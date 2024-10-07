export interface FileInfo {
    id: number;
    name: string;
    extension: string;
    page_data: string;
}

export interface HashChain {
    genesis_hash: string;
    domain_id: string;
    title: string;
    namespace: number;
    chain_height: number;
    revisions: Record<string, Revision>;  // Changed from Array<[string, Revision]>
}

export interface Revision {
    content: RevisionContent;
    metadata: RevisionMetadata;
    signature: string | null;
    witness: string | null;
}

export interface RevisionContent {
    file: FileContent | null;
    content: {
        file_hash?: string;
        [key: string]: string | undefined;
    };
    content_hash: string;
}

export interface FileContent {
    data: string;  // Base64 encoded string
    filename: string;
    size: number;
    comment: string;
}

export interface RevisionMetadata {
    domain_id: string;
    time_stamp: string;  // Changed from Timestamp object to string
    previous_verification_hash: string | null;
    metadata_hash: string;
    verification_hash: string;
}

export interface PageData {
    pages: HashChain[];
}