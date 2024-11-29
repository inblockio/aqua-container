import { Image } from "@chakra-ui/react";
import { ApiFileInfo } from "../models/FileInfo";
import { PageData } from "../models/PageData";
import { fileType } from "../utils/functions";


interface IFilePreview {
    fileInfo: ApiFileInfo
}

const FilePreview = ({ fileInfo }: IFilePreview) => {

    if (fileInfo == undefined) {
        return <div>

        </div>
    }
    const fileTypeInfo = fileType(fileInfo);

    const pageData: PageData = JSON.parse(fileInfo.page_data)
    console.log(pageData)

    if (pageData && pageData?.pages != null && pageData?.pages.length!! > 0) {
        const firstPage = pageData!.pages[0]; // Get the first page
        const firstRevisionKey = Object.keys(firstPage.revisions)[0]; // Get the first revision key
        const firstRevision = firstPage.revisions[firstRevisionKey]; // Get the first revision
        const fileContent = firstRevision.content.file; // Get file content

        if (fileContent && fileTypeInfo === "Image") {
            const base64String = `data:image/png;base64,${fileContent.data}`;
            return <Image src={base64String} borderRadius={'xl'} fit={'contain'} />
        }
        else if (fileContent && fileTypeInfo === "Video") {
            const base64String = `data:video/mp4;base64,${fileContent.data}`;
            return (
                <video
                    controls
                    width="100%"
                    style={{ borderRadius: '12px' }}
                >
                    <source src={base64String} type="video/mp4" />
                    Your browser does not support the video tag.
                </video>
            );
        }
    }
    return <div >
        <Image id="base64Image" alt="Base64 Image" src="/images/preview.jpg" />
    </div>
}

export default FilePreview