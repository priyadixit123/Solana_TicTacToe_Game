use anchor_lang::prelude::*;
use anchor_spl::token :: { self, Token, TokenAccount, Transfer};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2oJZQrTFcRFpYcVzEwfzwYW63eoznrJXQSWZ4ykiyztY");

#[program]
pub fn tic_tac_toe {
    use super::*;
    pub fn create_game(ctx: Context<CreateGame>, wager_amount: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.player_x = *ctx.accounts.player.key;
        game.player_o = Pubkey::default();
        game.board = [0; 9];
        game.turn = 1;
        game.status = 0;
        game.wager = wager_amount;
         
        Ok(())
    }
    pub fn join_game (ctx:Context<JoinGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        require!(game.player_o == Pubkey::default(), GameError::GameAlreadystarted);
        game.player_o = *ctx.accounts.player.key;
        Ok(())
    }

    pub fn play_move (ctx:Context<PlayMove>, position : u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let player = ctx.accounts.player.key();

        require!(position < 9, GameError::InvalidPosition);
        require!(game.board[position as unsize]== 0, GameError::positionOccupied);
        require!(game.status == 0, GameError::GameEnded);

        if game.turn == 1 
        {
            require!(player == game.player_x, GameError::NotYourTurn);
            game.board[position as unsize] = 1;
            game.trun = 2;

        } 
        else {
            require!(player == game.player_o,GameError::NotYourTurn);
            game.board[position as unsize] = 2;
            game.turn = 1;
        }

        if let some (winner) = check_winner (&game.board){
            game.status = winner;
        }

        Ok(())
    }

    pub fn claim_reward(ctx:Context<ClaimReward>) -> Result<()>{
        let game = &mut ctx.accounts.game;
        require!(game.status ! = 0, GameError::GameNotOver);
        let winner = match game.status {
            1 => game.player_x,
            2 => game.player_o,
            _ => return Err(GameError::DrawNoReward.into()),

        };

        require!(ctx.accounts.player.key() == winner, GameError::NotWinner);

        let total_wager = game.wager. checked_mul(2). unwarp();
        token::transfer(ctx.accounts
        .into_transfer_context(),
        total_wager,

         )?;
         Ok(())
    }
}

#[derive(Accounts)]
#[instruction (wager_amount:u64)]
pub struct CreateGame<'info> {
    
    #[account(init, payer = player, space = 8 + Game::LEN)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGame<'info>{
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct PlayMove<'info>{
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player:Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward <'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}


impl <'info> ClaimReward <'info> {
    fn into_transfer_context(&self) ->CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer{
                from : self.vault.to_account_info(),
                to: self.winner_token_account.to_account_info(),
                authority: self.player.to_account_info(),
            },
        )
    }
    
}

#[account]
pub struct Game {
    pub player_x: Pubkey,
    pub player_o : Pubkey,
    pub board:[u8; 9],
    pub turn :u8,
    pub status : u8,
    pub wager: u64,
}

impl  Game {
    pub const LEN : unsize = 32 + 32 + 9 + 1 + 1 + 8;
} 

#[error_code]

pub enum GameError {
    #[msg("Invalid board position.")]
    InvalidPosition,
     #[msg("That position is already taken.")]
    PositionOccupied,
     #[msg("It is not your turn.")]
    NotYourTurn,
     #[msg("The game is already over.")]
    GameEnded,
     #[msg("The Game is not Over Yet.")]
    GameNotOver,
    #[msg("Only the winner can Claim the Reward.")]
    NotWinner,
    #[msg(" Game Already Started.")]
    GameAlreadystarted,
    #[msg("No reward for draw.")]
    DrawNoReward,

}

fn check_winner (board: &[u8; 9]) -> Option<u8> {
    let wins = [
        [0,1,2],
        [3,4,5],
        [6,7,8],
        [0,3,6],
        [1,4,7],
        [2,5,8],
        [0,4,8],
        [2,4,6],
    ];

    for win in wins.iter() {
        if board [win[0]] !=0 && board [win[0]==board[win[1]] && board[win[1]] == board [win[2]]{
            return Some(board [win[0]]);

        }
    }

    if board.iter().all(|&x|x!=0) {
        return some(3);
    }

    None
}

    
