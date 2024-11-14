import { useState } from "react"
import {
    DrawerActionTrigger,
    DrawerBackdrop,
    DrawerBody,
    DrawerCloseTrigger,
    DrawerContent,
    DrawerFooter,
    DrawerHeader,
    DrawerRoot,
    DrawerTitle,
    DrawerTrigger,
} from "../drawer"
import { Button } from "../button"
import { LuEye, LuFileSignature, LuGlasses, LuShare2 } from "react-icons/lu"
import { Text } from "@chakra-ui/react"

const CustomDrawer = () => {
    const [open, setOpen] = useState(false)

    return (
        <DrawerRoot open={open} size={{ base: 'full', md: 'xl' }} onOpenChange={(e) => setOpen(e.open)}>
            <DrawerBackdrop />
            <DrawerTrigger asChild>
                <Button size={'xs'} colorPalette={'green'} variant={'subtle'} w={'80px'}>
                    <LuEye />
                    Details
                </Button>
            </DrawerTrigger>
            <DrawerContent>
                <DrawerHeader>
                    <DrawerTitle>FileName.json</DrawerTitle>
                </DrawerHeader>
                <DrawerBody>
                    <Text color={'gray.800'} _dark={{ color: 'white' }}>
                        Aqua Chain Details
                    </Text>
                </DrawerBody>
                <DrawerFooter>
                    <DrawerActionTrigger asChild>
                        <Button variant="outline" size={'sm'}>Close</Button>
                    </DrawerActionTrigger>
                    <Button size={'sm'}>
                        <LuShare2 />
                        Share
                    </Button>
                    <Button size={'sm'}>
                        <LuGlasses />
                        Witness
                    </Button>
                    <Button size={'sm'}>
                        <LuFileSignature />
                        Sign
                    </Button>
                </DrawerFooter>
                <DrawerCloseTrigger />
            </DrawerContent>
        </DrawerRoot>
    )
}

export default CustomDrawer