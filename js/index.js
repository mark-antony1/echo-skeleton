const {
  Connection,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
} = require("@solana/web3.js");
const anchor = require("@project-serum/anchor");

const BN = require("bn.js");

const main = async () => {
  var args = process.argv.slice(2);
  const programId = new PublicKey(args[0]);
  const echo = args[1];

  const connection = new Connection("https://api.devnet.solana.com/");

  const feePayer = new Keypair();
  const echoBuffer = new Keypair();

  console.log("Requesting Airdrop of 1 SOL...");
  await connection.requestAirdrop(feePayer.publicKey, 2e9);
  console.log("Airdrop received");

  let createIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: echoBuffer.publicKey,
    /** Amount of lamports to transfer to the created account */
    lamports: await connection.getMinimumBalanceForRentExemption(echo.length),
    /** Amount of space in bytes to allocate to the created account */
    space: echo.length,
    /** Public key of the program to assign as the owner of the created account */
    programId: programId,
  });

  const idx = Buffer.from(new Uint8Array([0]));
  const messageLen = Buffer.from(new Uint8Array((new BN(echo.length)).toArray("le", 4)));
  const message = Buffer.from(echo, "ascii");

  let echoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: echoBuffer.publicKey,
        isSigner: false,
        isWritable: true,
      },
    ],
    programId: programId,
    data: Buffer.concat([idx, messageLen, message]),
  });

  let tx = new Transaction();
  tx.add(createIx).add(echoIx);

  let txid = await sendAndConfirmTransaction(
    connection,
    tx,
    [feePayer, echoBuffer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

  data = (await connection.getAccountInfo(echoBuffer.publicKey, "confirmed")).data;
  console.log("Echo Buffer Text:", data.toString());

  /// START OF SECOND TEST
  console.log("SECOND TEST")
  const idx2 = Buffer.from(new Uint8Array([1]));

  const bufferSeed = new BN(40);
  const bufferSeedByteArray = new Uint8Array(bufferSeed.toArray("le", 8))
  const arbitraryBufferByteLength = 100
  const [authorizedBuffer, authorizedBufferBump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from('authority'), 
      feePayer.publicKey.toBuffer(), 
      Buffer.from(bufferSeedByteArray)
    ],
    programId
  );

  let echoIx2 = new TransactionInstruction({
    keys: [
      {
        pubkey: authorizedBuffer,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: programId,
    data: Buffer.concat([
      idx2, 
      Buffer.from(new Uint8Array(bufferSeed.toArray("le", 8))),
      Buffer.from(new Uint8Array((new BN(arbitraryBufferByteLength)).toArray("le", 8)))
    ]),
  });

  
  let tx2 = new Transaction();
  tx2.add(echoIx2);

  let txid2 = await sendAndConfirmTransaction(
    connection,
    tx2,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid2}?cluster=devnet`);

  data = (await connection.getAccountInfo(authorizedBuffer, "confirmed"));
  // console.log("authorized buffer data:", data.());

  // START OF THIRD TEST
  console.log("THIRD TEST")
  const idx3 = Buffer.from(new Uint8Array([2]));
  const messageLen2 = Buffer.from(new Uint8Array((new BN(echo.length)).toArray("le", 4)));
  const message2 = Buffer.from(echo, "ascii");

  let echoIx3 = new TransactionInstruction({
    keys: [
      {
        pubkey: authorizedBuffer,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
    ],
    programId: programId,
    data: Buffer.concat([
      idx3, 
      messageLen2, message2
    ]),
  });

  
  let tx3 = new Transaction();
  tx3.add(echoIx3);

  let txid3 = await sendAndConfirmTransaction(
    connection,
    tx3,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid3}?cluster=devnet`);

  data = (await connection.getAccountInfo(authorizedBuffer, "confirmed")).data;
  console.log("authorized buffer data:", data.toString());

    // START OF FOURTH TEST
    console.log("FOURTH TEST")
    const idx4 = Buffer.from(new Uint8Array([3]));
    const messageLen4 = Buffer.from(new Uint8Array((new BN(echo.length)).toArray("le", 4)));
    const message4 = Buffer.from(echo, "ascii");
  
    let echoIx4 = new TransactionInstruction({
      keys: [
        {
          pubkey: authorizedBuffer,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: feePayer.publicKey,
          isSigner: true,
          isWritable: false,
        },
      ],
      programId: programId,
      data: Buffer.concat([
        idx3, 
        messageLen4, message4
      ]),
    });
  
    
    let tx4 = new Transaction();
    tx3.add(echoIx3);
  
    let txid4 = await sendAndConfirmTransaction(
      connection,
      tx4,
      [feePayer],
      {
        skipPreflight: true,
        preflightCommitment: "confirmed",
        commitment: "confirmed",
      }
    );
    console.log(`https://explorer.solana.com/tx/${txid4}?cluster=devnet`);
  
    data = (await connection.getAccountInfo(authorizedBuffer, "confirmed")).data;
    console.log("authorized buffer data:", data.toString());
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
