"use client"

import { useSearchParams } from 'next/navigation'
import React, { useEffect, useState } from 'react'
import { ReactReader } from 'react-reader'

export default function ReaderPage() {
    // get access token
    const token = localStorage.getItem('accessToken') || ""

    // get book ID
    const searchParams = useSearchParams()
    const bookId = searchParams?.get('bookId')
    const extension = searchParams?.get('extension');

    // get book url
    const [location, setLocation] = useState<string | number>(0)
    const [bookUrl, setBookUrl] = useState<string>('')
    const [isLoading, setIsLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        const fetchBook = async () => {
            try {
                setIsLoading(true)
                const response = await fetch(`http://localhost:port/books/download/?bookId=${bookId}&extension=${extension}`, {
                    headers: {
                        'Authorization': `Bearer ${token}` // 替换为实际的访问令牌
                    }
                })
                if (!response.ok) {
                    throw new Error('Failed to fetch book')
                }
                const data = await response.json()
                setBookUrl(data.url)
                console.log(bookUrl)
            } catch (err: any) {
                setError(err.message)
            } finally {
                setIsLoading(false)
            }
        }

        if (bookId) {
            fetchBook()
        }
    }, [bookId, extension, token])

    if (isLoading) return <div className="flex justify-center items-center min-h-screen">Loading...</div>

    if (error) return <div className="flex justify-center items-center min-h-screen">Error: {error}</div>

    //console.log(extension)
    return (
        <div style={{ height: '100vh' }}>
            {extension === 'pdf' ? (
                //test 
                // window.location.href = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
                window.location.href = bookUrl
            ) : (
                <ReactReader
                    url={bookUrl}
                    location={location}
                    locationChanged={(epubcfi: string) => setLocation(epubcfi)}
                />
            )}
        </div>
    )

}
