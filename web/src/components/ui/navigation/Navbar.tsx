import { Box, HStack, Image } from "@chakra-ui/react"
import Settings from "../settings"
import ConnectWallet from "./ConnectWallet"


const Navbar = () => {
    return (
        <div>
            <Box bg={{base: 'rgb(188 220 255 / 22%)', _dark: 'rgba(0, 0, 0, 0.3)'}} h={'70px'}>
                <HStack h={'100%'} px={"4"} justifyContent={'space-between'}>
                    <Image src="/images/logo.png" maxH={'60%'} />
                    <HStack h={'100%'} justifyContent={'space-between'}>
                        <ConnectWallet />
                        <Settings />
                    </HStack>
                </HStack>
            </Box>
        </div>
    )
}

export default Navbar