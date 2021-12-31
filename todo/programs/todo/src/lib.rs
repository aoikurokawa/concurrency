use anchor_lang::prelude::*;

declare_id!("6NnowxYkaDjz7sto1RnuWgHreXZh7pNhHmNLL355m88U");

pub mod errors;

#[program]
pub mod todo {
    use super::*;
    pub fn new_list(
        ctx: Context<NewList>,
        name: String,
        capacity: u16,
        account_bump: u8,
    ) -> ProgramResult {
        // create a new account
        let list = &mut ctx.accounts.list;
        list.list_owner = *ctx.accounts.user.to_account_info().key;
        list.bump = account_bump;
        list.name = name;
        list.capacity = capacity;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct TodoList {
    pub list_owner: Pubkey,
    pub bump: u8,
    pub capacity: u16,
    pub name: String,
    pub lines: Vec<Pubkey>,
}

impl TodoList {
    fn space(name: &str, capacity: u16) -> usize {
        // discriminator + owner pubkey + bump + capacity
        8 + 32 + 1 + 2 + 4 + name.len() + 4 + (capacity as usize) * std::mem::size_of::<Pubkey>()
    }
}

fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 {
        &b[0..32]
    } else {
        b
    }
}

#[derive(Accounts)]
#[instruction(name: String, capacity: u16, list_bump: u8)]
pub struct NewList<'info> {
    #[account(init, payer = user, space = TodoList::space(&name, capacity), seeds=[b"todolist", user.to_account_info().key.as_ref(), name_seed(&name)], bump=list_bump)]
    pub list: Account<'info, TodoList>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
