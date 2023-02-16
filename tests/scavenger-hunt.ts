import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey"
import { Keypair } from "@solana/web3.js"
import { assert } from "chai"
import { ScavengerHunt } from "../target/types/scavenger_hunt"
const fs = require("fs")

describe("scavenger-hunt", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.ScavengerHunt as Program<ScavengerHunt>

  const rawdata = fs.readFileSync(
    "tests/keys/fun8eenPrVMJtiQNE7q1iBVDNuY2Lbnc3x8FFgCt43N.json"
  )
  const keyData = JSON.parse(rawdata)
  const eventOrganizer = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(keyData)
  )

  const gameId = Keypair.generate().publicKey

  const [userStatePDA] = findProgramAddressSync(
    [gameId.toBuffer(), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("Initialized", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(gameId).rpc()

    const userState = await program.account.userState.fetch(userStatePDA)
  })

  it("Check In", async () => {
    const location = Keypair.generate().publicKey
    // Add your test here.
    const tx = await program.methods
      .checkIn(gameId, location)
      .accounts({ eventOrganizer: eventOrganizer.publicKey })
      .signers([eventOrganizer])
      .rpc()

    const userState = await program.account.userState.fetch(userStatePDA)
    assert.isTrue(userState.lastLocation.equals(location))
  })

  it("Check In Again", async () => {
    const location = Keypair.generate().publicKey
    // Add your test here.
    const tx = await program.methods
      .checkIn(gameId, location)
      .accounts({ eventOrganizer: eventOrganizer.publicKey })
      .signers([eventOrganizer])
      .rpc()

    const userState = await program.account.userState.fetch(userStatePDA)
    assert.isTrue(userState.lastLocation.equals(location))
  })
})
