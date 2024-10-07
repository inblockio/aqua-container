import {FileInfo} from "./models/FileInfo";
import {PageData} from "./models/PageData";

export const imageTypes = ["image/png", "image/jpeg", "image/gif", "image/svg+xml"];
export const documentTypes = ["application/pdf", "application/msword", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"];
export const musicTypes = ["audio/mpeg", "audio/wav"];
export const videoTypes = ["video/mp4", "video/mpeg", "video/webm"];
export  function filterFilesByType(files: FileInfo[], fileType: "image" | "document" | "music" | "video"): FileInfo[] {
    switch (fileType) {
        case "image":
            return files.filter(file => imageTypes.includes(file.extension));
        case "document":
            return files.filter(file => documentTypes.includes(file.extension));
        case "music":
            return files.filter(file => musicTypes.includes(file.extension));
        case "video":
            return files.filter(file => videoTypes.includes(file.extension));
        default:
            return [];
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