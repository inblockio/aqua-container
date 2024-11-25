"use client"

import { Card, CardBody, FormatByte, Group, Kbd, Table, Text } from "@chakra-ui/react"
import {
    ActionBarContent,
    ActionBarRoot,
    ActionBarSelectionTrigger,
    ActionBarSeparator,
} from "./action-bar"
import { Button } from "./button"
import { Checkbox } from "./checkbox"
import { useState } from "react"
import { useStore } from "zustand"
import appStore from "../../store"
import { getFileCategory, getLastRevisionVerificationHash, sumFileContentSize, timeToHumanFriendly } from "../../utils/functions"
import { getTimestampSafe } from "../../models/PageData"
import { DeleteAquaChain, DownloadAquaChain, SignAquaChain, WitnessAquaChain } from "../aqua_chain_actions"
import ChainDetails from "./navigation/CustomDrawer"
import { Alert } from "./alert"


const FilesTable = () => {
    const { files } = useStore(appStore)
    const [selection, setSelection] = useState<string[]>([])

    const hasSelection = selection.length > 0
    const indeterminate = hasSelection && selection.length < files.length

    const rows = files?.map((item: any) => (
        <Table.Row
            key={item.id}
            data-selected={selection.includes(item.fileName) ? "" : undefined}
        >
            <Table.Cell>
                <Checkbox
                    top="1"
                    aria-label="Select File"
                    checked={selection.includes(item.id.toString())}
                    onCheckedChange={(changes) => {
                        setSelection((prev) =>
                            changes.checked
                                ? [...prev, item.id.toString()]
                                : selection.filter((id) => id !== item.id.toString()),
                        )
                    }}
                />
            </Table.Cell>
            <Table.Cell minW={'180px'} maxW={'180px'} textWrap={'wrap'}>{item.name}</Table.Cell>
            <Table.Cell minW={'120px'} maxW={'120px'} textWrap={'wrap'}>{getFileCategory(item.extension)}</Table.Cell>
            <Table.Cell minW={'140px'} maxW={'140px'} textWrap={'wrap'}>
                {timeToHumanFriendly(getTimestampSafe(JSON.parse(item.page_data)))}
            </Table.Cell>
            <Table.Cell minW={'100px'} maxW={'100px'} textWrap={'wrap'}>
                <FormatByte value={sumFileContentSize(JSON.parse(item.page_data))} />
            </Table.Cell>
            <Table.Cell minW={'220px'} maxW={'220px'} textWrap={'wrap'}>
                <Group alignItems={'start'} flexWrap={'wrap'}>
                    <DownloadAquaChain file={item} />
                    <ChainDetails fileInfo={item} />
                    <WitnessAquaChain filename={item.name} lastRevisionVerificationHash={getLastRevisionVerificationHash(JSON.parse(item.page_data))} />
                    <SignAquaChain filename={item.name} lastRevisionVerificationHash={getLastRevisionVerificationHash(JSON.parse(item.page_data))} />
                    <DeleteAquaChain filename={item.name} />
                </Group>
            </Table.Cell>
        </Table.Row>
    ))

    return (
        <Card.Root px={0} borderRadius={'2xl'}>
            <Card.Header>
                <Text fontWeight={500} fontSize={'2xl'}>Files</Text>
            </Card.Header>
            <CardBody px={0}>
                <Table.ScrollArea>
                    <Table.Root borderRadius={'2xl'} borderCollapse={'collapse'} borderSpacing={'4'}>
                        <Table.Header>
                            <Table.Row>
                                <Table.ColumnHeader w="6">
                                    <Checkbox
                                        top="1"
                                        aria-label="Select all rows"
                                        checked={indeterminate ? "indeterminate" : selection.length > 0}
                                        onCheckedChange={(changes) => {
                                            setSelection(
                                                changes.checked ? files.map((item: any) => item.id.toString()) : [],
                                            )
                                        }}
                                    />
                                </Table.ColumnHeader>
                                <Table.ColumnHeader fontWeight={600} fontSize={{ base: 'sm', md: 'md' }}>File Name</Table.ColumnHeader>
                                <Table.ColumnHeader fontWeight={600} fontSize={{ base: 'sm', md: 'md' }}>Type</Table.ColumnHeader>
                                <Table.ColumnHeader fontWeight={600} fontSize={{ base: 'sm', md: 'md' }}>Uploaded At</Table.ColumnHeader>
                                <Table.ColumnHeader fontWeight={600} fontSize={{ base: 'sm', md: 'md' }}>File Size</Table.ColumnHeader>
                                <Table.ColumnHeader fontWeight={600} fontSize={{ base: 'sm', md: 'md' }}>Action</Table.ColumnHeader>
                            </Table.Row>
                        </Table.Header>
                        <Table.Body>
                            {rows}
                            {rows.length === 0 ?
                                <Alert title="No Data">
                                    <Table.Cell colSpan={6}>
                                        Please upload some files or import an Aqua Chain
                                    </Table.Cell>
                                </Alert>
                                : null}
                        </Table.Body>
                    </Table.Root>
                </Table.ScrollArea>

                <ActionBarRoot open={hasSelection}>
                    <ActionBarContent>
                        <ActionBarSelectionTrigger>
                            {selection.length} selected
                        </ActionBarSelectionTrigger>
                        <ActionBarSeparator />
                        <Button variant="outline" size="sm">
                            Delete <Kbd>⌫</Kbd>
                        </Button>
                        <Button variant="outline" size="sm">
                            Share <Kbd>T</Kbd>
                        </Button>
                    </ActionBarContent>
                </ActionBarRoot>
            </CardBody>
        </Card.Root>
    )
}



export default FilesTable