import { Box, GridItem, Group, Image, SimpleGrid, StatRoot, Text, VStack } from "@chakra-ui/react";
import { StatHelpText, StatLabel } from "./ui/stat";
import { LuDot } from "react-icons/lu";
import { UiFileTypes } from "../models/UiFileTypes";
import { ApiFileInfo } from "../models/FileInfo";
import { filterFilesByType, humanReadableFileSize, sumFileContentSize } from "../utils/functions";
import { PageData } from "../models/PageData";
import { useStore } from "zustand";
import appStore from "../store";

interface IStatistic {
    title: string
    tagline: string
    size: string
    image: string
    files: string
}

const Statistic = (props: IStatistic) => {

    return (
        <StatRoot shadow={'sm'} borderRadius={'xl'} px={'4'} py="6" h={'100%'}>
            <Group>
                <Box>
                    <Image src={props.image} w={'60px'} h={'60px'} />
                </Box>
                <VStack gap={0} alignItems={'start'}>
                    <Text fontWeight={500} fontSize={'xl'}>{props.title}</Text>
                    <StatLabel fontWeight={400} fontSize={'small'}>{props.tagline}</StatLabel>
                </VStack>
            </Group>
            <StatHelpText mt={'2'} fontWeight={400} fontSize={'medium'} display={'inline-flex'}>
                {props.size}
                <LuDot />
                {`${props.files} Files`}
            </StatHelpText>
        </StatRoot>
    )
}

const calculateTotalFilesSize = (files: ApiFileInfo[]) => {
    let size = 0;
    for (const element of files) {
        const pageData: PageData = JSON.parse(element.page_data);
        // Debug the structure
        // debugPageDataStructure(pageData);

        let currentSize = sumFileContentSize(pageData)
        size += currentSize
    }
    return size
}

const getFileTypeProportions = (files: ApiFileInfo[]) => {

    let fileTypes = ["image", "document", "music", "video"];

    // let filesUiState: Array<UiFileTypes> = [];
    let filesUiState: Record<string, UiFileTypes> = {}

    let totalFilesSize = calculateTotalFilesSize(files)

    for (const element of fileTypes) {

        let fileItemData = filterFilesByType(files, element)

        let size = 0;
        for (const element of fileItemData) {
            const pageData: PageData = JSON.parse(element.page_data);
            // Debug the structure
            // debugPageDataStructure(pageData);

            let currentSize = sumFileContentSize(pageData)
            size += currentSize
        }

        // console.log("element " + element + " length " + fileItemData.length + "  file " + element + " size  " + size);
        let percentage = size / totalFilesSize * 100
        let usingText = `Using ${percentage.toFixed(2)}% of storage`
        let hSize = humanReadableFileSize(size)

        let item: UiFileTypes = {
            name: element,
            usingText: usingText,
            size: hSize,
            totalFiles: `${fileItemData.length}`
        }

        // filesUiState.push(item)
        filesUiState[element] = item

    }
    return filesUiState
}


export default function Statistics() {
    const { files } = useStore(appStore)
    let storageUsage = getFileTypeProportions(files)
    
    return (
        <SimpleGrid columns={{ base: 2, md: 2, lg: 4 }} gapX={'4'} gapY={'4'}>
            <GridItem>
                <Statistic title="Documents" image="/images/stats/doc.png" files={storageUsage?.document?.totalFiles ?? "-"} size={storageUsage?.document?.size ?? "-"} tagline={storageUsage?.document?.usingText ?? "-"} />
            </GridItem>
            <GridItem>
                <Statistic title="Images" image="/images/stats/image.png" files={storageUsage?.image?.totalFiles ?? "-"} size={storageUsage?.image?.size ?? "-"} tagline={storageUsage?.image?.usingText ?? "-"} />
            </GridItem>
            <GridItem>
                <Statistic title="Music" image="/images/stats/music.png" files={storageUsage?.music?.totalFiles ?? "-"} size={storageUsage?.music?.size ?? "-"} tagline={storageUsage?.music?.usingText ?? "-"} />
            </GridItem>
            <GridItem>
                <Statistic title="Videos" image="/images/stats/video.png" files={storageUsage?.video?.totalFiles ?? "-"} size={storageUsage?.video?.size ?? "-"} tagline={storageUsage?.video?.usingText ?? "-"} />
            </GridItem>
            {/* <GridItem>
                <Statistic title="Others" image="/images/stats/folder.png" files={storageUsage?.document?.totalFiles ?? "-"} size={storageUsage?.document?.size ?? "-"} tagline={storageUsage?.document?.usingText ?? "-"} />
            </GridItem> */}
        </SimpleGrid>
    )
}