import { Card, createListCollection, Group, HStack, IconButton, Input, Text, VStack } from "@chakra-ui/react"
import { LuSettings } from "react-icons/lu"
import { DialogActionTrigger, DialogBody, DialogCloseTrigger, DialogContent, DialogFooter, DialogHeader, DialogRoot, DialogTitle, DialogTrigger } from "./dialog"
import { Field } from "./field"
import { useState } from "react"
import { RadioCardItem, RadioCardRoot } from "./radio-card"
import { ColorModeButton } from "./color-mode"
import { ENDPOINTS } from "../../utils/constants"
import axios from "axios"
import { useStore } from "zustand"
import appStore from "../../store"
import { toaster } from "./toaster"
import { Button } from "./button"

const networks = createListCollection({
    items: [
        { label: "Mainnet", value: "mainnet" },
        { label: "Sepolia", value: "sepolia" },
        { label: "Holesky", value: "holesky" },
    ],
})

const fileModes = createListCollection({
    items: [
        { label: "Public", value: "public" },
        { label: "Private", value: "private" },
    ],
})

const SettingsForm = () => {
    const { setUserProfile, user_profile } = useStore(appStore)
    const [activeNetwork, setActiveNetwork] = useState<string>(user_profile.network)
    const [domain, setDomain] = useState<string>(user_profile.domain)
    const [mode, setMode] = useState<string>(user_profile.fileMode)
    const [contract, setContract] = useState<string>(user_profile.contractAddress ?? "0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611")

    const updateUserProfile = async () => {
        const formData = new URLSearchParams();
        formData.append('chain', activeNetwork);
        formData.append('domain', domain);
        formData.append('mode', mode);
        formData.append('contract', contract);
        formData.append('theme', 'light');


        const response = await axios.post(ENDPOINTS.UPDATE_USER_PROFILE, formData, {
            headers: {
                // 'Content-Type': 'application/x-www-form-urlencoded'
            }
        });

        if (response.status === 200) {
            setUserProfile({
                contractAddress: contract,
                network: activeNetwork,
                domain: domain,
                fileMode: mode
            })

            toaster.create({
                description: "Settings saved successfully",
                type: "success",
            })

        }
    }

    return (
        <VStack alignItems={'start'} gapY={'6'}>
            <Card.Root w={'100%'} shadow={'sm'} borderRadius={'SM'}>
                <Card.Body p={'4px'} px={'20px'}>
                    <Group justifyContent={'space-between'} w="100%">
                        <Text>Theme</Text>
                        <ColorModeButton />
                    </Group>
                </Card.Body>
            </Card.Root>
            <Field invalid={false} label="Domain Name" errorText="This field is required">
                <Input placeholder="Domain Name" value={domain} onChange={e => setDomain(e.currentTarget.value)} />
            </Field>
            <Field invalid={false} label="Contract Address" errorText="This field is required" >
                <Input placeholder="Contract Address" value={contract} onChange={e => setContract(e.currentTarget.value)} />
            </Field>
            <Field invalid={false} label="Select Network" errorText="This field is required" >
                <RadioCardRoot value={activeNetwork} onValueChange={e => setActiveNetwork(e.value)}>
                    <HStack align="stretch">
                        {networks.items.map((item) => (
                            <RadioCardItem
                                borderRadius={'xl'}
                                label={item.label}
                                key={item.value}
                                value={item.value}
                            />
                        ))}
                    </HStack>
                </RadioCardRoot>
            </Field>
            <Field invalid={false} label="Default File Mode" helperText="Is a file public or private" errorText="This field is required">
                <RadioCardRoot defaultValue="public" value={mode} onValueChange={e => setMode(e.value)}>
                    <HStack align="stretch">
                        {fileModes.items.map((item) => (
                            <RadioCardItem
                                borderRadius={'xl'}
                                label={item.label}
                                key={item.value}
                                value={item.value}
                            />
                        ))}
                    </HStack>
                </RadioCardRoot>
            </Field>
            <Group>
                <Button onClick={updateUserProfile}>Save</Button>
            </Group>
        </VStack>
    )
}

const DeleteFiles = () => {
    const [deleting, setDeleting] = useState(false)
    const { setFiles } = useStore(appStore)

    const deleteFile = async () => {
        try {

            setDeleting(true)
            const response = await axios.get(ENDPOINTS.DELETE_ALL_FILES, {
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                }
            });

            if (response.status === 200) {
                setFiles([])
                toaster.create({
                    description: "Files cleared successfully",
                    type: "success"
                })
            }
            setDeleting(false)
        }
        catch (e: any) {
            toaster.create({
                description: `Failed to clear files ${e}`,
                type: "error"
            })
            setDeleting(false)
        }
    }

    return (
        <Button loading={deleting} colorPalette={'red'} borderRadius={'md'} variant={'subtle'} onClick={deleteFile}>Delete all Data</Button>
    )
}

const Settings = () => {

    return (
        <div>
            <DialogRoot size={{ md: 'md', smDown: 'full' }} placement={'top'}>
                <DialogTrigger asChild>
                    <IconButton
                        onClick={() => { }}
                        variant="ghost"
                        aria-label="Toggle color mode"
                        size="sm"
                        css={{
                            _icon: {
                                width: "5",
                                height: "5",
                            },
                        }}
                    >
                        <LuSettings />
                    </IconButton>
                </DialogTrigger>
                <DialogContent borderRadius={{ base: 0, md: 'xl' }}>
                    <DialogHeader>
                        <DialogTitle>Settings</DialogTitle>
                    </DialogHeader>
                    <DialogBody >
                        <SettingsForm />
                    </DialogBody>
                    <DialogFooter>
                        <HStack w={'100%'} justifyContent={'space-between'}>
                            <DeleteFiles />
                            <HStack>
                                <DialogActionTrigger asChild>
                                    <Button variant="outline">Cancel</Button>
                                </DialogActionTrigger>
                            </HStack>
                        </HStack>
                    </DialogFooter>
                    <DialogCloseTrigger />
                </DialogContent>
            </DialogRoot>
        </div>
    )
}

export default Settings