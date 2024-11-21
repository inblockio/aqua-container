import { ethers } from "ethers";
import { ApiFileInfo } from "../models/FileInfo";
import { PageData } from "../models/PageData";
import { documentTypes, ENDPOINTS, imageTypes, musicTypes, videoTypes } from "./constants";
import { AvatarGenerator } from 'random-avatar-generator';

export function formatCryptoAddress(address?: string, start: number = 10, end: number = 4): string {
    if (!address) return "NO ADDRESS"
    if (address?.length < (start + end)) {
        // throw new Error(`Address must be at least ${start + end} characters long.`);
        return address
    }

    const firstPart = address?.slice(0, start);
    const lastPart = address?.slice(-end);
    return `${firstPart}...${lastPart}`;
}

export function remove0xPrefix(input: string): string {
    // Check if the input string starts with '0x'
    if (input.startsWith('0x')) {
        // Remove the prefix and return the remaining string
        return input.slice(2);
    }
    // Return the original string if it doesn't start with '0x'
    return input;
}

export function getCookie(name: string) {
    const value = `; ${document.cookie}`;
    const parts: any = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(";").shift();
    return null;
}

export function setCookie(name: string, value: string, expirationTime: Date) {
    const expirationDate = new Date(expirationTime);
    // For UTC cookie settings
    // document.cookie = `${name}=${value}; expires=${expirationDate.toUTCString()}; path=/; Secure; SameSite=Strict`;
    document.cookie = `${name}=${value}; expires=${expirationDate}; path=/; Secure; SameSite=Strict`;
}

export async function getCurrentNetwork() {
    if (typeof window.ethereum !== 'undefined') {
        try {
            const chainId = await window.ethereum.request({ method: 'eth_chainId' });
            console.log("Current chain ID:", chainId);
            return chainId;
        } catch (error) {
            console.error("Error fetching chain ID:", error);
        }
    } else {
        console.error("MetaMask is not installed.");
    }
}

export async function switchNetwork(chainId: string) {
    // const chainId = '0x89'; // Example: Polygon Mainnet chain ID
    if (typeof window.ethereum !== 'undefined') {
        try {
            // Check if the network is already set
            await window.ethereum.request({
                method: 'wallet_switchEthereumChain',
                params: [{ chainId }],
            });
            console.log("Network switched successfully");
        } catch (error) {
            // If the network is not added, request MetaMask to add it

        }
    } else {
        console.error("MetaMask is not installed.");
    }
}


export async function fetchFiles(publicMetaMaskAddress: string): Promise<Array<ApiFileInfo>> {
    try {
        const query = await fetch(ENDPOINTS.EXPOLORER_FETCH_FILES, {
            method: 'GET',
            headers: {
                'metamask_address': publicMetaMaskAddress
            },
        });
        const response = await query.json()

        if (!query.ok) {
            throw new Error(`HTTP error! status: ${query.status}`);
        }

        let res = response;
        let logs: Array<string> = res.logs
        logs.forEach((item) => {
            console.log("**>" + item + "\n.")
        })

        // console.log("fetchFiles Response " + response.body)

        // Parse the response body as JSON
        const data = res.files;

        // Assuming the API returns an array of FileInfo objects
        const files: Array<ApiFileInfo> = data.map((item: any) => ({
            id: item.id,
            name: item.name,
            extension: item.extension,
            page_data: item.page_data,
            mode: item.mode,
            owner: item.owner,
        }));

        return files;
    } catch (error) {
        console.error("Error fetching files:", error);
        return [];
    }
}


export function getFileCategory(extension: string): string | null {
    // Remove the leading dot if present (e.g., ".png" becomes "png")
    // const ext = extension.startsWith('.') ? extension.slice(1).toLowerCase() : extension.toLowerCase();
    const extParts = extension.split('/')
    const ext = extParts[extParts.length - 1]

    // Map of file categories with extensions
    const fileCategories: Record<string, string> = {
        // Image
        jpg: "Image",
        jpeg: "Image",
        png: "Image",
        gif: "Image",
        svg: "Image",
        webp: "Image",
        bmp: "Image",
        ico: "Image",
        // Audio
        mp3: "Audio",
        wav: "Audio",
        ogg: "Audio",
        mp4: "Video",
        webm: "Video",
        // Documents
        pdf: "Document",
        doc: "Document",
        docx: "Document",
        xls: "Document",
        xlsx: "Document",
        ppt: "Document",
        pptx: "Document",
        txt: "Document",
        html: "Document",
        css: "Document",
        js: "Document",
        json: "Document",
        xml: "Document",
        zip: "Archive",
        rar: "Archive",
        "7z": "Archive",
    }

    // Loop through each category and look for the extension


    // Return null if not found
    return fileCategories[ext];
}


export function calculateContentSize(content: string | Buffer | Blob): number {
    if (typeof content === "string") {
        // For a string, return the number of bytes by encoding it into UTF-8
        return new TextEncoder().encode(content).length;
    }
    else if (Buffer.isBuffer(content)) {
        // For a Buffer, return its length directly (in bytes)
        return content.length;
    }
    else if (content instanceof Blob) {
        // For a Blob (File), return the size property (in bytes)
        return content.size;
    }

    throw new Error("Unsupported content type");
}


export function sumFileContentSize(pageData: PageData): number {
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

export function getTimestampSafe(pageData: PageData): string | null {
    return pageData.pages[0]?.revisions[Object.keys(pageData.pages[0]?.revisions || {})[0]]?.metadata.time_stamp;
}

export function timeToHumanFriendly(timestamp: string | undefined): string {
    if (!timestamp) {
        return '-'
    }
    // Extract the date components
    let year = timestamp.substring(0, 4);
    let month = Number(timestamp.substring(4, 6)) - 1; // Months are zero-indexed in JS
    let day = timestamp.substring(6, 8);
    let hours = timestamp.substring(8, 10);
    let minutes = timestamp.substring(10, 12);
    let seconds = timestamp.substring(12, 14);

    // Create a new Date object
    let date = new Date(Date.UTC(Number(year), month, Number(day), Number(hours), Number(minutes), Number(seconds)));

    // Format the date in a human-friendly way
    let options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'short', day: 'numeric' };
    return date.toLocaleDateString('en-US', options)
}


export const getLastRevisionVerificationHash = (pageData: PageData) => {
    const revisionHashes = Object.keys(pageData.pages[0].revisions)
    return pageData.pages[0].revisions[revisionHashes[revisionHashes.length - 1]].metadata.verification_hash
}

export function filterFilesByType(files: ApiFileInfo[], fileType: string): ApiFileInfo[] { // "image" | "document" | "music" | "video"

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


export function readJsonFile(file: File): Promise<any> {
    return new Promise((resolve, reject) => {
        if (file.type !== "application/json") {
            reject(new Error("The file is not a JSON file."));
            return;
        }

        const reader = new FileReader();
        reader.onload = () => {
            try {
                const json = JSON.parse(reader.result as string);
                resolve(json);
            } catch (error) {
                reject(new Error("Error parsing JSON content."));
            }
        };

        reader.onerror = () => {
            reject(new Error("Error reading the file."));
        };

        reader.readAsText(file);
    });
}

export const isJSONFile = (fileName: string) => {
    return fileName.trim().toLowerCase().endsWith('.json');
}

export function generateAvatar(_address: string) {
    let address = ethers.getAddress(_address)
    let generator = new AvatarGenerator()
    return generator.generateRandomAvatar(address)
}

// const b64toBlob = (b64Data: string, contentType = "", sliceSize = 512) => {
//     const byteCharacters = atob(b64Data);
//     const byteArrays = [];

//     for (let offset = 0; offset < byteCharacters.length; offset += sliceSize) {
//         const slice = byteCharacters.slice(offset, offset + sliceSize);

//         const byteNumbers = new Array(slice.length);
//         for (let i = 0; i < slice.length; i++) {
//             byteNumbers[i] = slice.charCodeAt(i);
//         }

//         const byteArray = new Uint8Array(byteNumbers);
//         byteArrays.push(byteArray);
//     }

//     const blob = new Blob(byteArrays, { type: contentType });
//     return blob;
// };