import {FileInfo} from "./models/FileInfo";
import {PageData, Timestamp} from "./models/PageData";

export const imageTypes = ["image/png", "image/jpeg", "image/gif", "image/svg+xml"];
export const documentTypes = ["application/pdf", "application/msword", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"];
export const musicTypes = ["audio/mpeg", "audio/wav"];
export const videoTypes = ["video/mp4", "video/mpeg", "video/webm"];

export function filterFilesByType(files: FileInfo[], fileType: string): FileInfo[] { // "image" | "document" | "music" | "video"

    switch (fileType) {
        case "image":
            return files.filter(file => {
                return imageTypes.includes(file.extension.replace(/\s+/g, ''))
            });
        case "document":
            return files.filter(file => documentTypes.includes(file.extension.replace(/\s+/g, '')));
        case "music":
            return files.filter(file => musicTypes.includes(file.extension.replace(/\s+/g, '')));
        case "video":
            return files.filter(file => videoTypes.includes(file.extension.replace(/\s+/g, '')));
        default:
            return [];
    }
}

export function fileType(file: FileInfo): string {
    if (imageTypes.includes(file.extension.replace(/\s+/g, ''))) {
        return "Image";
    } else if (documentTypes.includes(file.extension.replace(/\s+/g, ''))) {
        return "Document";
    } else if (musicTypes.includes(file.extension.replace(/\s+/g, ''))) {
        return "Image";
    } else if (videoTypes.includes(file.extension.replace(/\s+/g, ''))) {
        return "Video";
    } else {
        return "unknown";
    }
}

export function humanReadableFileSize(size: number): string {
    const units = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB'];
    let index = 0;

    // Convert size in bytes to the appropriate unit
    while (size >= 1024 && index < units.length - 1) {
        size /= 1024;
        index++;
    }

    // Return the size formatted with 2 decimal places, along with the appropriate unit
    return `${size.toFixed(2)} ${units[index]}`;
}

export function sumFileContentSizes(pageData: PageData): number {
    if (!pageData?.pages) return 0;

    return pageData.pages.reduce((totalSize, hashChain) => {
        if (!hashChain.revisions) return totalSize;

        const chainSize = Object.values(hashChain.revisions).reduce((chainTotal, revision) => {
            if (revision?.content?.file?.size) {
                return chainTotal + revision.content.file.size;
            }
            return chainTotal;
        }, 0);

        return totalSize + chainSize;
    }, 0);
}

// Helper function to debug the data structure
export function debugPageDataStructure(pageData: PageData): void {
    console.log('PageData structure:');
    console.log('Number of pages:', pageData.pages?.length);
    pageData.pages?.forEach((hashChain, index) => {
        console.log(`Page ${index}:`);
        console.log('  Revisions type:', typeof hashChain.revisions);
        console.log('  Revisions type:', JSON.stringify(hashChain.revisions));
        console.log('  Is array:', Array.isArray(hashChain.revisions));
        console.log('  Revisions length:', hashChain.revisions?.length);
        if (hashChain.revisions?.length > 0) {
            console.log('  First revision type:', typeof hashChain.revisions[0]);
        }
    });
}


export function capitalizeFirstLetter(str: string, forceLowerRest: boolean = false): string {
    if (!str) return str;

    const firstChar = str.charAt(0).toUpperCase();
    const restOfString = forceLowerRest
        ? str.slice(1).toLowerCase()
        : str.slice(1);

    return `${firstChar}${restOfString}`;
}


export  function timeToHumanFriendly(utcTime : string): string{
    // Extract the date components
    let year = utcTime.substring(0, 4);
    let month = utcTime.substring(4, 6) - 1; // Months are zero-indexed in JS
    let day = utcTime.substring(6, 8);
    let hours = utcTime.substring(8, 10);
    let minutes = utcTime.substring(10, 12);
    let seconds = utcTime.substring(12, 14);

// Create a new Date object
    let date = new Date(Date.UTC(Number(year), month, Number(day), Number(hours), Number(minutes), Number(seconds)));

// Format the date in a human-friendly way
    let options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'short', day: 'numeric' };
    return  date.toLocaleDateString('en-US', options)
}


export class DateFormatter {
    /**
     * Converts a Timestamp to a JavaScript Date object
     */
    static timestampToDate(timestamp: Timestamp): Date {
        return new Date(timestamp.seconds * 1000 + timestamp.nanos / 1000000);
    }

    /**
     * Formats a Timestamp into a human-friendly string
     * @param timestamp - The Timestamp to format
     * @param format - Optional format ('short', 'medium', 'long', 'full')
     * @returns Formatted date string
     */
    static formatTimestamp(
        timestamp: Timestamp,
        format: 'short' | 'medium' | 'long' | 'full' = 'medium'
    ): string {
        const date = this.timestampToDate(timestamp);

        const formatOptions: Record<string, Intl.DateTimeFormatOptions> = {
            short: {
                month: 'short',
                day: 'numeric',
                year: 'numeric'
            },
            medium: {
                month: 'short',
                day: 'numeric',
                year: 'numeric',
                hour: 'numeric',
                minute: 'numeric'
            },
            long: {
                month: 'long',
                day: 'numeric',
                year: 'numeric',
                hour: 'numeric',
                minute: 'numeric',
                second: 'numeric'
            },
            full: {
                weekday: 'long',
                month: 'long',
                day: 'numeric',
                year: 'numeric',
                hour: 'numeric',
                minute: 'numeric',
                second: 'numeric',
                timeZoneName: 'short'
            }
        };

        return date.toLocaleString('en-US', formatOptions[format]);
    }
}