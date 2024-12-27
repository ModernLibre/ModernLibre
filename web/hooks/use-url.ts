'use client'

import { useEffect, useState } from "react"
import { fetchBookUrl } from "@/lib/api"

export function useUrl(bookId: number, extension: string) {
    const [url, setUrl] = useState<string>("")
    const [uerror, setError] = useState<Error | null>(null)

    useEffect(() => {
        let mounted = true;
        async function getUrl() {
            // get book url
            try {
                const bookUrl = await fetchBookUrl(bookId, extension)
                if (mounted) setUrl(bookUrl)
            }
            catch (error) {
                if (mounted) setError(error as Error)
            }
        }

        getUrl()
        return () => {
            mounted = false
        }
    }, [bookId])

    return { url , uerror }
}