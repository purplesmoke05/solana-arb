use anchor_lang::prelude::*;
use arb::Side;

use solana_program::account_info::AccountInfo;
use anchor_spl::token::accessor::amount;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct AldrinSwapExtend<'info> {
    swap_program: AccountInfo<'info>,
    pool: AccountInfo<'info>,
    pool_signer: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    base_token_vault: AccountInfo<'info>,
    #[account(mut)]
    quote_token_vault: AccountInfo<'info>,
    #[account(mut)]
    fee_pool_token_account: AccountInfo<'info>,
    wallet_authority: Signer<'info>,
    #[account(mut)]
    user_base_token_account: AccountInfo<'info>,
    #[account(mut)]
    user_quote_token_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct AldrinV2SwapExtend<'info> {
    swap_program: AccountInfo<'info>,
    pool: AccountInfo<'info>,
    pool_signer: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    base_token_vault: AccountInfo<'info>,
    #[account(mut)]
    quote_token_vault: AccountInfo<'info>,
    #[account(mut)]
    fee_pool_token_account: AccountInfo<'info>,
    wallet_authority: Signer<'info>,
    #[account(mut)]
    user_base_token_account: AccountInfo<'info>,
    #[account(mut)]
    user_quote_token_account: AccountInfo<'info>,
    curve: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct LifinityTokenSwapExtend<'info> {
    swap_program: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amm: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    source_info: AccountInfo<'info>,
    #[account(mut)]
    destination_info: AccountInfo<'info>,
    #[account(mut)]
    swap_source: AccountInfo<'info>,
    #[account(mut)]
    swap_destination: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    fee_account: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    pyth_account: AccountInfo<'info>,
    pyth_pc_account: AccountInfo<'info>,
    #[account(mut)]
    config_account: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct WhirlPoolSwapExtend<'info> {
    swap_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    token_authority: Signer<'info>,
    #[account(mut)]
    whirlpool: AccountInfo<'info>,
    #[account(mut)]
    token_owner_account_a: AccountInfo<'info>,
    #[account(mut)]
    token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    token_owner_account_b: AccountInfo<'info>,
    #[account(mut)]
    token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    tick_array_0: AccountInfo<'info>,
    #[account(mut)]
    tick_array_1: AccountInfo<'info>,
    #[account(mut)]
    tick_array_2: AccountInfo<'info>,
    oracle: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}


#[derive(Accounts)]
pub struct MercurialExchangeExtend<'info> {
    swap_program: AccountInfo<'info>,
    swap_state: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    pool_authority: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    source_token_account: AccountInfo<'info>,
    #[account(mut)]
    destination_token_account: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct SaberSwapExtend<'info> {
    swap_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    swap: AccountInfo<'info>,
    swap_authority: AccountInfo<'info>,
    user_authority: Signer<'info>,
    #[account(mut)]
    input_user_account: AccountInfo<'info>,
    #[account(mut)]
    input_token_account: AccountInfo<'info>,
    #[account(mut)]
    output_user_account: AccountInfo<'info>,
    #[account(mut)]
    output_token_account: AccountInfo<'info>,
    #[account(mut)]
    fees_token_account: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct SenchaExchangeExtend<'info> {
    swap_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    #[account(mut)]
    swap: AccountInfo<'info>,
    user_authority: AccountInfo<'info>,
    #[account(mut)]
    input_user_account: AccountInfo<'info>,
    #[account(mut)]
    input_token_account: AccountInfo<'info>,
    #[account(mut)]
    input_fees_account: AccountInfo<'info>,
    #[account(mut)]
    output_user_account: AccountInfo<'info>,
    #[account(mut)]
    output_token_account: AccountInfo<'info>,
    #[account(mut)]
    output_fees_account: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct TokenSwapExtend<'info> {
    token_swap_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    swap_source: AccountInfo<'info>,
    #[account(mut)]
    swap_destination: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    pool_fee: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct CremaTokenSwapExtend<'info> {
    swap_program: AccountInfo<'info>,
    #[account(mut)]
    pool: AccountInfo<'info>,
    pool_signer: AccountInfo<'info>,
    #[account(mut)]
    user_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    user_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pool_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    pool_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pool_ticks_account: AccountInfo<'info>,
    wallet_authority: Signer<'info>,
    token_program: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct StepTokenSwapExtend<'info> {
    token_swap_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    swap_source: AccountInfo<'info>,
    #[account(mut)]
    swap_destination: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    pool_fee: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[derive(Accounts)]
pub struct CropperTokenSwapExtend<'info> {
    token_swap_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    swap: AccountInfo<'info>,
    swap_state: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    swap_source: AccountInfo<'info>,
    #[account(mut)]
    swap_destination: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    pool_fee: AccountInfo<'info>,
    pub cpi_program: Program<'info, arb::program::Jupiter>
}

#[program]
pub mod arb_master {
    use super::*;

    pub fn crema_token_swap_cpi(ctx: Context<CremaTokenSwapExtend>,in_amount:u64, use_chain_amount: bool, minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.user_source_token_account).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::CremaTokenSwap{
            swap_program: ctx.accounts.swap_program.clone(),
            pool: ctx.accounts.pool.clone(),
            pool_signer: ctx.accounts.pool_signer.clone(),
            user_source_token_account: ctx.accounts.user_source_token_account.clone(),
            user_destination_token_account: ctx.accounts.user_destination_token_account.clone(),
            pool_source_token_account: ctx.accounts.pool_source_token_account.clone(),
            pool_destination_token_account: ctx.accounts.pool_destination_token_account.clone(),
            pool_ticks_account: ctx.accounts.pool_ticks_account.clone(),
            wallet_authority: ctx.accounts.wallet_authority.to_account_info(),
            token_program: ctx.accounts.token_program.clone(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::crema_token_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn cropper_token_swap_cpi(ctx: Context<CropperTokenSwapExtend>, in_amount:u64, use_chain_amount: bool,minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.source).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::CropperTokenSwap{
            token_swap_program: ctx.accounts.token_swap_program.clone(),
            token_program: ctx.accounts.token_program.clone(),
            swap: ctx.accounts.swap.clone(),
            swap_state: ctx.accounts.swap_state.clone(),
            authority: ctx.accounts.authority.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.clone(),
            source: ctx.accounts.source.clone(),
            swap_source: ctx.accounts.swap_source.clone(),
            swap_destination: ctx.accounts.swap_destination.clone(),
            destination: ctx.accounts.destination.clone(),
            pool_mint: ctx.accounts.pool_mint.clone(),
            pool_fee: ctx.accounts.pool_fee.clone(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::cropper_token_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn step_token_swap_cpi(ctx: Context<StepTokenSwapExtend>, in_amount:u64, use_chain_amount: bool,minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.source).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::StepTokenSwap{
            token_swap_program: ctx.accounts.token_swap_program.clone(),
            token_program: ctx.accounts.token_program.clone(),
            swap: ctx.accounts.swap.clone(),
            authority: ctx.accounts.authority.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.clone(),
            source: ctx.accounts.source.clone(),
            swap_source: ctx.accounts.swap_source.clone(),
            swap_destination: ctx.accounts.swap_destination.clone(),
            destination: ctx.accounts.destination.clone(),
            pool_mint: ctx.accounts.pool_mint.clone(),
            pool_fee: ctx.accounts.pool_fee.clone(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::step_token_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn token_swap_cpi(ctx: Context<TokenSwapExtend>,in_amount:u64, use_chain_amount: bool, minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.source).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::TokenSwap{
            token_swap_program: ctx.accounts.token_swap_program.clone(),
            token_program: ctx.accounts.token_program.clone(),
            swap: ctx.accounts.swap.clone(),
            authority: ctx.accounts.authority.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.clone(),
            source: ctx.accounts.source.clone(),
            swap_source: ctx.accounts.swap_source.clone(),
            swap_destination: ctx.accounts.swap_destination.clone(),
            destination: ctx.accounts.destination.clone(),
            pool_mint: ctx.accounts.pool_mint.clone(),
            pool_fee: ctx.accounts.pool_fee.clone(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::token_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn sencha_exchange_cpi(ctx: Context<SenchaExchangeExtend>,in_amount:u64, use_chain_amount: bool, minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.input_token_account).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::SenchaExchange{
            swap_program: ctx.accounts.swap_program.clone(),
            token_program: ctx.accounts.token_program.clone(),
            swap: ctx.accounts.swap.clone(),
            user_authority: ctx.accounts.user_authority.clone(),
            input_user_account: ctx.accounts.input_user_account.clone(),
            input_token_account: ctx.accounts.input_token_account.clone(),
            input_fees_account: ctx.accounts.input_fees_account.clone(),
            output_user_account: ctx.accounts.output_user_account.clone(),
            output_token_account: ctx.accounts.output_token_account.clone(),
            output_fees_account: ctx.accounts.output_fees_account.clone(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::sencha_exchange(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn saber_swap_cpi(ctx: Context<SaberSwapExtend>,in_amount:u64, use_chain_amount: bool, minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.input_token_account).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::SaberSwap{
            swap_program: ctx.accounts.swap_program.clone(),
            token_program: ctx.accounts.token_program.clone(),
            swap: ctx.accounts.swap.clone(),
            swap_authority: ctx.accounts.swap_authority.clone(),
            user_authority: ctx.accounts.user_authority.to_account_info(),
            input_user_account: ctx.accounts.input_user_account.clone(),
            input_token_account: ctx.accounts.input_token_account.clone(),
            output_user_account: ctx.accounts.output_user_account.clone(),
            output_token_account: ctx.accounts.output_token_account.clone(),
            fees_token_account: ctx.accounts.fees_token_account.clone()
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::saber_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn mercurial_exchange_cpi<'info>(ctx: Context<'_, '_, '_, 'info, MercurialExchangeExtend<'info>>, in_amount:u64, use_chain_amount: bool, minimum_out_amount: u64) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.source_token_account).unwrap(),
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::MercurialExchange{
            swap_program: ctx.accounts.swap_program.clone(),
            swap_state: ctx.accounts.swap_state.clone(),
            token_program: ctx.accounts.token_program.clone(),
            pool_authority: ctx.accounts.pool_authority.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.to_account_info(),
            source_token_account: ctx.accounts.source_token_account.clone(),
            destination_token_account: ctx.accounts.destination_token_account.clone()
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.cpi_program.to_account_info(), 
            cpi_accounts
        ).with_remaining_accounts(ctx.remaining_accounts.to_vec());
        
        arb::cpi::mercurial_exchange(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)

    }

    pub fn whirl_pool_swap_cpi(ctx: Context<WhirlPoolSwapExtend>, in_amount:u64, use_chain_amount: bool, minimum_out_amount: u64, a_to_b: bool) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=> match a_to_b {
                true => amount(&ctx.accounts.token_owner_account_a).unwrap(),
                false => amount(&ctx.accounts.token_owner_account_b).unwrap()
            }
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::WhirlpoolSwap{
                swap_program: ctx.accounts.swap_program.clone(),
                token_program: ctx.accounts.token_program.clone(),
                token_authority: ctx.accounts.token_authority.to_account_info().clone(),
                whirlpool: ctx.accounts.whirlpool.clone(),
                token_owner_account_a: ctx.accounts.token_owner_account_a.clone(),
                token_vault_a: ctx.accounts.token_vault_a.clone(),
                token_owner_account_b: ctx.accounts.token_owner_account_b.clone(),
                token_vault_b: ctx.accounts.token_vault_b.clone(),
                tick_array0: ctx.accounts.tick_array_0.clone(),
                tick_array1: ctx.accounts.tick_array_1.clone(),
                tick_array2: ctx.accounts.tick_array_2.clone(),
                oracle: ctx.accounts.oracle.clone()
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::whirlpool_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, a_to_b, 0)
    }

    pub fn lifinity_token_swap_cpi(ctx: Context<LifinityTokenSwapExtend>, in_amount: u64, use_chain_amount: bool, minimum_out_amount: u64) -> Result<()> {

        let _in_amount = match use_chain_amount {
            true=>amount(&ctx.accounts.source_info).unwrap(),
            false=>in_amount
        };


        let cpi_accounts = arb::cpi::accounts::LifinityTokenSwap{
            swap_program: ctx.accounts.swap_program.clone(),
            authority: ctx.accounts.authority.clone(),
            amm: ctx.accounts.amm.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.to_account_info().clone(),
            source_info: ctx.accounts.source_info.clone(),
            destination_info: ctx.accounts.destination_info.clone(),
            swap_source: ctx.accounts.swap_source.clone(),
            swap_destination: ctx.accounts.swap_destination.clone(),
            pool_mint: ctx.accounts.pool_mint.clone(),
            fee_account: ctx.accounts.fee_account.clone(),
            token_program: ctx.accounts.token_program.clone(),
            pyth_account: ctx.accounts.pyth_account.clone(),
            pyth_pc_account: ctx.accounts.pyth_pc_account.clone(),
            config_account: ctx.accounts.config_account.clone()
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::lifinity_token_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, 0)
    }

    pub fn aldrin_v2_swap_cpi(ctx: Context<AldrinV2SwapExtend>, in_amount: u64, use_chain_amount: bool, minimum_out_amount: u64, side: Side) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>match side {
                Side::Ask => amount(&ctx.accounts.user_base_token_account).unwrap(),
                Side::Bid => amount(&ctx.accounts.user_quote_token_account).unwrap(),
            }
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::AldrinV2Swap{
            swap_program: ctx.accounts.swap_program.clone(),
            pool: ctx.accounts.pool.clone(),
            pool_signer: ctx.accounts.pool_signer.clone(),
            pool_mint: ctx.accounts.pool_mint.clone(),
            base_token_vault: ctx.accounts.base_token_vault.clone(),
            quote_token_vault: ctx.accounts.quote_token_vault.clone(),
            fee_pool_token_account: ctx.accounts.fee_pool_token_account.clone(),
            wallet_authority: ctx.accounts.wallet_authority.to_account_info(),
            user_base_token_account: ctx.accounts.user_base_token_account.clone(),
            user_quote_token_account: ctx.accounts.user_quote_token_account.clone(),
            curve: ctx.accounts.curve.clone(),
            token_program: ctx.accounts.token_program.clone()
        };
        
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::aldrin_v2_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, side, 0)
    }

    pub fn aldrin_swap_cpi(ctx: Context<AldrinSwapExtend>, in_amount: u64, use_chain_amount: bool, minimum_out_amount: u64, side: Side) -> Result<()> {
        let _in_amount = match use_chain_amount {
            true=>match side {
                Side::Ask => amount(&ctx.accounts.user_base_token_account).unwrap(),
                Side::Bid => amount(&ctx.accounts.user_quote_token_account).unwrap(),
            }
            false=>in_amount
        };
        let cpi_accounts = arb::cpi::accounts::AldrinSwap{
            swap_program: ctx.accounts.swap_program.clone(),
            pool: ctx.accounts.pool.clone(),
            pool_signer: ctx.accounts.pool_signer.clone(),
            pool_mint: ctx.accounts.pool_mint.clone(),
            base_token_vault: ctx.accounts.base_token_vault.clone(),
            quote_token_vault: ctx.accounts.quote_token_vault.clone(),
            fee_pool_token_account: ctx.accounts.fee_pool_token_account.clone(),
            wallet_authority: ctx.accounts.wallet_authority.to_account_info(),
            user_base_token_account: ctx.accounts.user_base_token_account.clone(),
            user_quote_token_account: ctx.accounts.user_quote_token_account.clone(),
            token_program: ctx.accounts.token_program.clone()
        };
        
        let cpi_ctx = CpiContext::new(ctx.accounts.cpi_program.to_account_info(), cpi_accounts);
        arb::cpi::aldrin_swap(cpi_ctx, Some(_in_amount), minimum_out_amount, side, 0)
    }
}
