'use client'

import Image from 'next/image'
import { Card, CardContent, CardFooter } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Download, Star } from 'lucide-react'
import Link from 'next/link'
import { useBooks } from '@/hooks/use-books'
import type { Book } from '@/hooks/use-book'

interface BookCardProps {
  book: Book
}

function BookCard({ book }: BookCardProps) {
  console.log('Book metadata:', book); // 打印书的元信息
  return (
    <Card className="overflow-hidden">
      <Link href={`/library/${book.id}`}>
        <CardContent className="p-0">
          <div className="aspect-[2/3] relative group">
            <Image
              src={book.cover_url || '/static/books/none.jpg'}
              alt={book.title}
              fill
              className="object-cover transition-transform group-hover:scale-105"
            />
            <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
              <Button variant="secondary" size="icon" className="mr-2">
                <Download className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardContent>
        <CardFooter className="flex flex-col items-start p-4">
          <h3 className="font-semibold line-clamp-1">{book.title}</h3>
          {book.author && (
            <p className="text-sm text-muted-foreground">{book.author}</p>
          )}
          {book.rating && (
            <div className="flex items-center gap-1 mt-1">
              <Star className="h-4 w-4 fill-primary text-primary" />
              <span className="text-sm">{book.rating}</span>
            </div>
          )}
        </CardFooter>
      </Link>
    </Card>
  )
}

// Temporary mock data until we implement the full API
const mockBooks: Book[] = [
  {
    id: 1001,
    title: "The Design of Everyday Things",
    author: "Don Norman",
    description: "A powerful primer on how design serves as the interface between objects and users",
    rating: 4.9,
    status: 1,
    cover_url: "/static/books/design-of-everyday-things.jpg",
    added_date: new Date().toISOString()
  },
  {
    id: 1002,
    title: "Atomic Habits",
    author: "James Clear",
    description: "Tiny Changes, Remarkable Results",
    rating: 4.8,
    status: 1,
    cover_url: "/static/books/atomic-habits1.jpg",
    added_date: new Date().toISOString()
  },

  {
    id: 1003,
    title: "Deep Work",
    author: "Cal Newport",
    description: "Somthing about deep work",
    rating: 4.7,
    status: 1,
    cover_url: "/static/books/deep-work1.jpg",
    added_date: new Date().toISOString()
  },
  {
    id: 1004,
    title: "The Psychology of Money",
    author: "Morgan Housel",
    description: "Somthing about deep work",
    rating: 4.7,
    status: 1,
    cover_url: "/static/books/psychology-of-money.jpg",
    added_date: new Date().toISOString()
  },
  // {
  //   id: 1323,
  //   title: "The Rust Programming Language",
  //   author: "Steve Klabnik and Carol Nichols",
  //   description: "A comprehensive guide to Rust programming.",
  //   status: 1,
  //   rating: 4.5,
  //   added_date: "2023-11-04T20:48:30"
  // }
]

export function BookGrid() {
  const { books, loading, error } = useBooks()

  if (loading) {
    return (
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
        {[...Array(10)].map((_, i) => (
          <Card key={i} className="animate-pulse">
            <div className="aspect-[2/3] bg-muted" />
            <CardFooter className="flex flex-col items-start p-4">
              <div className="h-4 w-3/4 bg-muted rounded mb-2" />
              <div className="h-3 w-1/2 bg-muted rounded" />
            </CardFooter>
          </Card>
        ))}
      </div>
    )
  }

  // Combine API books with mock books, or just show mock books if API fails
  const displayBooks = error ? mockBooks : [...books]

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
      {displayBooks.map((book) => (
        <BookCard key={book.id} book={book} />
      ))}
    </div>
  )
}