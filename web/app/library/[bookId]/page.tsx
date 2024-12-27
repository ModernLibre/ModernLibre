'use client'

import { useBook } from '@/hooks/use-book'
import { Card, CardContent, CardFooter } from "@/components/ui/card"
import { Download, Star, ArrowLeft, Eye } from 'lucide-react'
import Image from 'next/image'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import { Button } from '@/components/ui/button'
import { useUrl } from '@/hooks/use-url'
import React from 'react'

export default function BookPage() {
  const params = useParams()
  const bookId = parseInt(params.bookId as string)
  const { book, loading, error } = useBook(bookId)
  const { url, uerror } = book ? useUrl(bookId, book.extension as string) : { url: null, uerror: null }

  if (loading) {
    return <div className="flex justify-center items-center min-h-screen">Loading...</div>
  }

  if (error) {
    return <div className="flex justify-center items-center min-h-screen">Error: {error.message}</div>
  }

  if (!book) {
    return <div className="flex justify-center items-center min-h-screen">Book not found</div>
  }

  // url 获取失败的情况下阻止跳转
  const handlePreviewClick = (e: React.MouseEvent<HTMLAnchorElement, MouseEvent>) => {
    if (uerror) {
      e.preventDefault()
      alert('URL 获取失败，无法在线预览');
    }
  }

  const handleDownloadClick = (e: React.MouseEvent<HTMLAnchorElement, MouseEvent>) => {
    if (uerror) {
      e.preventDefault()
      alert('URL 获取失败，无法下载');
    }
  }

  return (
    <div className="min-h-screen bg-background p-8">
      <div className="container max-w-4xl mx-auto">
        <Link
          href="/library"
          className="inline-flex items-center gap-2 text-muted-foreground hover:text-foreground mb-8"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to Library
        </Link>

        <div className="grid md:grid-cols-2 gap-8">
          <Card className="overflow-hidden">
            <CardContent className="p-0">
              <div className="aspect-[2/3] relative">
                <Image
                  src={book.cover_url || '/static/books/none.jpg'}
                  alt={book.title}
                  fill
                  className="object-cover"
                />
              </div>
            </CardContent>
          </Card>

          <div className="space-y-4">
            <h1 className="text-3xl font-bold">{book.title}</h1>
            {book.author && (
              <p className="text-xl text-muted-foreground">{book.author}</p>
            )}
            {book.description && (
              <p className="text-muted-foreground">{book.description}</p>
            )}
            {book.rating && (
              <div className="flex items-center gap-1">
                <Star className="h-5 w-5 fill-primary text-primary" />
                <span className="text-lg">{book.rating}</span>
              </div>
            )}
            <div className="w-[50%] pt-4 flex flex-col space-y-2">
              <div className='w-full'>
                <Link href={`/reader?url=${url}&extension=${book.extension}`} onClick={handlePreviewClick} className="w-full block">
                  <Button className="w-full">
                    <Eye className="mr-2 h-4 w-4" />
                    Read Online
                  </Button>
                </Link>
              </div>
              <div className='w-full'>
                <Link href={`${url}`} download={`${book.title}.${book.extension}`} onClick={handleDownloadClick} className="w-full block">
                  <Button className="w-full">
                    <Download className="mr-2 h-4 w-4" />
                    Download Book
                  </Button>
                </Link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}