/**
 * Navbar component
 * 
 * This component is responsible for rendering the navbar at the top of the page.
 * It also handles the logic for connecting and disconnecting web3 wallets.
 */

import { 
    Box, Button, Flex, Heading, Modal, ModalBody, ModalCloseButton, 
    ModalContent, ModalFooter, ModalHeader, 
    ModalOverlay, Spacer, Text, useDisclosure 
} from '@chakra-ui/react'
import { useWallet, useAllWallets } from 'useink'

export default function Navbar() {
    const wallets = useAllWallets()
    const { isConnected, connect, disconnect, setAccount, accounts, account } = useWallet()
    const { isOpen: isWalletPickerOpen, onOpen: onPickerOpen, onClose: onPickerClose } = useDisclosure()

    const _getAbbreviatedAccountAddress = (address: string) => {
        return [
            address.split('').splice(0, 6).join(''),
            '...',
            address.split('').splice(address.length - 6, address.length).join('')
        ].join('')
    }

    const _renderWallets = () => {
        return (
            <ul>
                {wallets.map(w => (
                    <li style={{ textAlign: 'left', listStyle: 'none' }} key={w.title}>
                        {
                            w.installed ? (
                                <Button
                                    onClick={() => {
                                        connect(w.extensionName)
                                        onPickerClose()
                                    }}
                                    style={{ margin: 10, padding: 10 }}
                                    variant="outlined"
                                    height={'64px'}
                                    width={'100%'}
                                    colorScheme='blue'
                                    marginTop={'10px'}
                                    leftIcon={
                                        <img src={w.logo.src} alt={w.logo.alt} width={28} height={28} />
                                    }
                                >
                                    <Flex>
                                        <Text>
                                            Connect to {w.title}
                                        </Text>
                                    </Flex>
                                </Button>
                            ) : (
                                <Button
                                    height={'64px'}
                                    width={'100%'}
                                    variant='solid'
                                    onClick={() => window.open(w.installUrl, '_blank')}
                                    marginTop={'10px'}
                                    leftIcon={
                                        <img src={w.logo.src} alt={w.logo.alt} width={28} height={28} />
                                    }
                                >
                                    <Flex>
                                        <Text>
                                            Install {w.title}
                                        </Text>
                                    </Flex>
                                </Button>
                            )
                        }
                    </li>
                ))}
            </ul>
        )
    }

    const _renderWalletPickerModal = () => (
        <Modal isOpen={isWalletPickerOpen} onClose={onPickerClose}>
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>
                    Pick a wallet to connect
                </ModalHeader>
                <ModalCloseButton />
                <ModalBody>
                    { _renderWallets() }
                </ModalBody>
                <ModalFooter>
                    <Button colorScheme='purple' mr={3} onClick={onPickerClose}>
                        Close
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
    )

    return (
        <Box
            style={{ padding: '10px' }}
            bgColor={'purple.700'}
        >
            <Flex>
                <Heading color={'white'}>
                    Game Slice
                </Heading>
                <Spacer />
                {
                    isConnected && (
                        <Box>
                            <Button
                                onClick={disconnect}
                                colorScheme='orange'
                            >
                                Disconnect &nbsp;
                                <strong>
                                    { account ? `${ _getAbbreviatedAccountAddress(account.address) }` : '' }
                                </strong>
                            </Button>
                        </Box>
                    )
                }
                {
                    !isConnected && (
                        <Box>
                            <Button color="inherit" onClick={() => onPickerOpen()}>
                                Connect Wallet
                            </Button>
                        </Box>
                    )
                }
                { _renderWalletPickerModal() }
            </Flex>
        </Box>
    )
}
