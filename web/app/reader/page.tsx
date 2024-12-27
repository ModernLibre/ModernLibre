"use client"
import { useSearchParams } from 'next/navigation'
import React, { useEffect, useState } from 'react'
import { ReactReader } from 'react-reader'

export default function ReaderPage() {
    const [location, setLocation] = useState<string | number>(0)
    const params = useSearchParams()
    const url = params.get('url')
    // console.log(url)
    const extension = params.get('extension')

    useEffect(() => {
        if (extension === 'pdf' && url) {
            // window.location.href = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
            window.location.href = url;
        }
    }, [extension, url]);

    if (extension === 'pdf') {
        return <div className="flex justify-center items-center min-h-screen">LoadingPDF...</div>
    }

    //console.log(extension)
    return (
        <div style={{ height: '100vh' }}>
            <ReactReader
                url={url as string}
                location={location}
                locationChanged={(epubcfi: string) => setLocation(epubcfi)}
            />
        </div>
    )

}
