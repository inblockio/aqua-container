import { useEffect, useState } from "react";
import { Button } from "../button";
import { DialogBody, DialogCloseTrigger, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "../dialog";
import { Center, Dialog, Spacer, Text, VStack } from "@chakra-ui/react";
import { LuMessageCircleWarning, LuWorm } from "react-icons/lu";
import { useStore } from 'zustand'
import appStore from '../../../store'
import { Alert } from "../alert";
import axios from "axios";
import { toaster } from "../toaster";

export default function VersionAndDisclaimer() {
    //   const {  es, avatar, setAvatar, setUserProfile, backend_url } = useStore(appStore);

    const { backend_url } = useStore(appStore)
    const [isOpen, setIsOpen] = useState(false);
    const [loading, setLoading] = useState(false);
    //   const [connectionState, setConnectionState] = useState<"idle" | "connecting" | "success" | "error">("idle");
    //   const [message, setMessage] = useState<string | null>(null);
    //   // const [avatar, setAvatar] = useState("")
    //   const [_progress, setProgress] = useState(0);

    //   const iconSize = "120px";

    //   const resetState = () => {
    //     setConnectionState("idle");
    //     setProgress(0);
    //   };


    useEffect(()=>{

     
    })


    const fetchVersionDetails=async ()=>{
        try{
            const url = `${backend_url}/explorer_witness_file`
    
            const response = await axios.post(url, formData, {
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                }
            })
        
            const res = await response.data;
            // let logs: Array<string> = res.logs
            // // logs.forEach((item) => {
            // //     console.log("**>" + item + "\n.")
            // // })
        
            if (response.status === 200) {
    
            }
          }catch (e){
            toaster.create({
                description: "Error fetching version details",
                type: "error"
            })
          }
    }

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
                        <Text fontFamily={"monospace"}>Container Api Version : </Text>
                        <Text fontFamily={"monospace"}>Container Web Version : {packageJson.version} </Text>
                        <Spacer height={30} />

                        <Alert status="info" title="" variant="solid"   >
                            This is prototype software,use it with caution.
                        </Alert>
                        <Button borderRadius={"md"} loading={loading} onClick={() => {

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
