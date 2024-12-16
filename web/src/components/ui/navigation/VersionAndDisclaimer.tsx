import { useEffect, useState } from "react";
import { Button } from "../button";
import { DialogBody, DialogCloseTrigger, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "../dialog";
import { Center, Dialog, Spacer, Text, VStack } from "@chakra-ui/react";
import { LuMessageCircleWarning } from "react-icons/lu";
import { useStore } from 'zustand'
import appStore from '../../../store'
import { Alert } from "../alert";
import axios from "axios";
import { toaster } from "../toaster";
import VersionDetails from "../../../models/VersionDetails";

export default function VersionAndDisclaimer() {
    //   const {  es, avatar, setAvatar, setUserProfile, backend_url } = useStore(appStore);

    const { backend_url } = useStore(appStore)
    const [isOpen, setIsOpen] = useState(false);
    const [versionDetails, setVersionDetails] = useState<VersionDetails>({
        backend: "1.2.X",
        frontend: "1.2.X"
    });
    





    const fetchVersionDetails = async () => {
        try {
            const url = `${backend_url}/version`

            const response = await axios.get(url)

            const res: VersionDetails = await response.data;


            if (response.status === 200) {
                setVersionDetails(res)
            }
        } catch (e:  unknown) {
            console.log("Error fetching version ", e)
            toaster.create({
                description: "Error fetching version details",
                type: "error"
            })
        }
    }

    useEffect(() => {
        fetchVersionDetails()

    },[])

    return (
        <Dialog.Root placement={"center"} size={"sm"} open={isOpen} onOpenChange={(details) => setIsOpen(details.open)}>
            <DialogTrigger asChild>
                <Button
                    size={"sm"}
                    borderRadius={"md"}
                    onClick={() => {
                        setIsOpen(true);
                        // !metamaskAddress && signAndConnect();
                    }}
                >
                    <LuMessageCircleWarning />
                    Info
                </Button>
            </DialogTrigger>
            <DialogContent borderRadius={"2xl"} overflow={"hidden"}>
                <DialogHeader py={"3"} px={"5"} bg={{ base: "rgb(188 220 255 / 22%)", _dark: "rgba(0, 0, 0, 0.3)" }}>
                    <DialogTitle fontWeight={500} color={"gray.800"} _dark={{ color: "white" }}>
                        Product Infomation
                    </DialogTitle>
                </DialogHeader>
                <DialogBody py={"8"} px={"5"}>

                    <VStack gap={5}>

                        <Center>
                            Product Verion Details
                        </Center>
                        <Text fontFamily={"monospace"}>Container Api Version : {versionDetails.backend}  </Text>
                        <Text fontFamily={"monospace"}>Container Web Version : {versionDetails.frontend} </Text>
                        <Spacer height={30} />

                        <Alert status="info" title="" variant="solid"   >
                            This is prototype software,use it with caution.
                        </Alert>
                        <Button borderRadius={"md"}  onClick={() => {
                            setIsOpen(!isOpen);
                        }}>
                            close
                            {/* <LuClose /> */}
                        </Button>
                    </VStack>

                </DialogBody>
                <DialogCloseTrigger />
            </DialogContent>
        </Dialog.Root>
    );
}
