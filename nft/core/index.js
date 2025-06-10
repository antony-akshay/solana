import { create, mplCore } from '@metaplex-foundation/mpl-core'
import {
    createGenericFile,
    generateSigner,
    signerIdentity,
    sol,
} from '@metaplex-foundation/umi'
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys'
import { base58 } from '@metaplex-foundation/umi/serializers'

const createNft = async () => {
    const umi = createUmi('https://api.devnet.solana.com')

    const signer = generateSigner(umi)
    umi.use(signerIdentity(signer))
    umi.use(mplCore())
    umi.use(irysUploader({ address: 'https://devnet.irys.xyz' }))

    console.log("Public Key:", umi.identity.publicKey.toString())

    // Airdrop
    console.log('Airdropping 1 SOL to identity')
    try {
        await umi.rpc.airdrop(umi.identity.publicKey, sol(1))
        let balance = await umi.rpc.getBalance(umi.identity.publicKey)
        while (balance.basisPoints.toNumber() < 1_000_000_000) {
            console.log("Waiting for airdrop to complete...")
            await new Promise(resolve => setTimeout(resolve, 2000))
            balance = await umi.rpc.getBalance(umi.identity.publicKey)
        }
    } catch (e) {
        console.error("Airdrop failed. You may need to use https://faucet.solana.com manually.")
        return
    }

    const metadataUri = 'https://arweave.net/some-valid-json-link' // <-- replace with valid JSON

    const asset = generateSigner(umi)

    console.log('Creating NFT...')
    const tx = await create(umi, {
        asset,
        name: 'My NFT',
        uri: metadataUri,
    }).sendAndConfirm(umi)

    const signature = base58.deserialize(tx.signature)[0]

    console.log('\n✅ NFT Created')
    console.log('🔗 Solana Explorer:')
    console.log(`https://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log('\n🔍 Metaplex Core Explorer:')
    console.log(`https://core.metaplex.com/explorer/${asset.publicKey}?env=devnet`)
}

createNft()
