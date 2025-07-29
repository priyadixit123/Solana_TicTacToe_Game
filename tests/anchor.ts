i  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TicTacToeSolana as anchor.Program<TicTacToeSolana>;
  
mport BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
 import * as anchor from "@coral-xyz/anchor";
 import { Program } from "@coral-xyz/anchor";
 import { assert } from "chai";
 import { TicTacToe } from "../target/types/tic_tac_toe";
import type { TicTacToeSolana } from "../target/types/tic_tac_toe_solana";

 describe ("tic_tac_toe", ()=>{
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;

  const playerX = provider.wallet;
  const playerO = anchor.web3.Keypair.generate();

  let gameAccount : anchor.web3.Keypair;

  it("Creates a new game", async ()=>{
    gameAccount = anchor.web3.Keypair.generate();

    await program.methods 
    .createGame(new anchor.BN(1000))
    .accounts ({
      game: gameAccount.publicKey,
      player: playerX.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([gameAccount])
    .rpc();

    const game = await program.account.game.fetch(gameAccount.publicKey);
    assert.ok(game.playerX.equals(playerX.publicKey));
    assert.ok(game.playerO.equals(anchor.web3.PublicKey.default));
    assert.equal(game.board.toString(), new Array(9).fill(0).toString());
    assert.equal(game.status, 0);
  });

  it("Player O joins the game", async()=>{
    const tx = await program.methods
    .joinGame()
    .accounts({
      game: gameAccount.publicKey,
      player: playerO.publicKey,
    })
    .signers([playerO])
    .rpc();

    const game = await program.account.game.fetch(gameAccount.publicKey);
    assert.ok(game.playerO.equals(playerO.publicKey));
  });

  it("Player X joins the game", async()=>
  {
     await program.methods
     .playMove(0)
     .accounts({
      game: gameAccount.publicKey,
      player : playerX.publicKey,
     })
    .rpc();
    const game = await program.account.game.fetch(gameAccount.publicKey);
    assert.equal(game.board[0], 1);
    assert.equal(game.turn, 2);

  });

  it("Player O makes a move", async()=>{
    await program.methods
    .playMove(1),
    .accounts ({
      game: gameAccount.publicKey,
      player: playerO.publicKey,
    })
    .signers([playerO])
    .rpc();

    const game = await program.account.game.fetch(gameAccount.publicKey);
    assert.equal(game.board[1], 2);
    assert.equal(game.turn,1);
  });

  it("Player X makes a move ", async()=>{
    await program.methods
    .playMove(3)
    .accounts({game: gameAccount.publicKey, player: playerX.publicKey})
    .rpc();

    await program.methods
    .playMove(2)
    .accounts({
      game: gameAccount.publicKey,
      player: playerO.publicKey,
    })
    .signers([playerO])
    .rpc();

    await program.methods
    .playMove(6)
    .accounts({game: gameAccount.publicKey, player: playerX.publicKey})
    .rpc();

    const game = await program.account.game.fetch(gameAccount.publicKey);
    assert.equal( game.status, 1);

  });

  it("Player X cannot play after game ended", async()=>{
    try {
      await program.methods
      .playMove(4)
      .accounts({ game : gameAccount.publicKey, player: playerX.publicKey})
      .rpc();
      assert.fail ("Should not allow move after game ends");

    }catch(err) {
      assert.include(err.message, "The game is already over");
    }
  });
 });