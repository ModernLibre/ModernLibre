import { promises } from 'node:dns';
import { cache } from 'react';

// 试用，后续应该统一配置到环境变量中
let backendUrl = 'http://localhost:8083';

const TIMEOUT_MS = 5000; // 5 seconds timeout

// Create an in-memory cache for books
const booksCache = new Map<number, any>();
let allBooksCache: any[] | null = null;
let lastFetchTimestamp = 0;
const CACHE_TTL = 5 * 60 * 1000; // 5 minutes cache TTL

// Utility function to handle fetch with timeout
async function fetchWithTimeout(url: string, options: RequestInit = {}) {
  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), TIMEOUT_MS);

  try {
    console.log('Fetching URL:', url);
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
    });
    clearTimeout(id);

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response;
  } catch (error) {
    clearTimeout(id);
    if (error instanceof Error) {
      if (error.name === 'AbortError') {
        throw new Error('Request timeout');
      }
    }
    throw error;
  }
}

// Cache-wrapped fetch book function
export const fetchBook = cache(async (id: number) => {
  // Check cache first
  const cachedBook = booksCache.get(id);
  const now = Date.now();
  
  if (cachedBook && (now - cachedBook.timestamp) < CACHE_TTL) {
    return cachedBook.data;
  }

  try {
    const response = await fetchWithTimeout(backendUrl+`/books/details/${id}`);
    const data = await response.json();
    
    // Update cache
    booksCache.set(id, {
      data,
      timestamp: now
    });
    
    return data;
  } catch (error) {
    console.error('Error fetching book:', error);
    throw error;
  }
});

// Cache-wrapped fetch books function
export const fetchBooks = cache(async () => {
  const now = Date.now();
  
  // Return cached data if it's still valid
  if (allBooksCache && (now - lastFetchTimestamp) < CACHE_TTL) {
    return allBooksCache;
  }
  
  try {
    const response = await fetchWithTimeout(backendUrl+`/books/list`);
    const data = await response.json();
    
    // Update cache
    allBooksCache = data;
    lastFetchTimestamp = now;
    
    // Also update individual book caches
    if (Array.isArray(data)) {
      data.forEach(book => {
        booksCache.set(book.id, {
          data: book,
          timestamp: now
        });
      });
    }
    
    return data;
  } catch (error) {
    console.error('Error fetching books:', error);
    throw error;
  }
});

// Function to clear cache (useful for mutations)
export function clearCache() {
  booksCache.clear();
  allBooksCache = null;
  lastFetchTimestamp = 0;
}

// Function to prefetch a book (useful for optimistic updates)
export async function prefetchBook(id: number) {
  try {
    await fetchBook(id);
  } catch (error) {
    console.error('Error prefetching book:', error);
  }
}

// Function to prefetch all books
export async function prefetchBooks() {
  try {
    await fetchBooks();
  } catch (error) {
    console.error('Error prefetching books:', error);
  }
}

// fetch book url function
export async function fetchBookUrl(id : number, extension : string): Promise<any> {
  try {
    const response = await fetchWithTimeout(`${window.location.origin}/api/books/download/${id}/${extension}`);
    const data = await response.json();
    return data;
  }
  catch (error) {
    console.error('Error fetching book url:', error);
    throw error;
  }
}