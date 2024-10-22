import { ApiFileInfo } from "../models/FileInfo";

export async function fetchFiles(publicMetaMaskAddress : string): Promise<Array<ApiFileInfo>> {
    try {
        const query = await fetch("http://127.0.0.1:3600/explorer_files", {
            method: 'GET',
            headers: {
                'public_key': publicMetaMaskAddress
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

        console.log("fetchFiles Response " + response.body)

        // Parse the response body as JSON
        const data = res.files;

        // Assuming the API returns an array of FileInfo objects
        const files: Array<ApiFileInfo> = data.map((item: any) => ({
            id: item.id,
            name: item.name,
            extension: item.extension,
            page_data: item.page_data,
            mode:item.mode,
            owner:item.owner,            
        }));

        return files;
    } catch (error) {
        console.error("Error fetching files:", error);
        return [];
    }
}