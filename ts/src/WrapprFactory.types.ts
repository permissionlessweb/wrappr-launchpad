/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.25.2.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type ExecuteMsg = {
  create_minter: CreateMinterMsgForWrapprMinterInitMsgExtension;
};
export type Timestamp = Uint64;
export type Uint64 = string;
export type Uint128 = string;
export interface CreateMinterMsgForWrapprMinterInitMsgExtension {
  collection_params: CollectionParams;
  init_msg: WrapprMinterInitMsgExtension;
}
export interface CollectionParams {
  code_id: number;
  info: CollectionInfo;
  name: string;
  symbol: string;
}
export interface CollectionInfo {
  creator: string;
  description: string;
  explicit_content?: boolean | null;
  external_link?: string | null;
  image: string;
  start_trading_time?: Timestamp | null;
}
export interface WrapprMinterInitMsgExtension {
  entity: string;
  jurisdiction: string;
  mint_price: Coin;
  payment_address?: string | null;
  per_address_limit: number;
  start_time: Timestamp;
  whitelist?: string | null;
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export interface InstantiateMsg {
  params: MinterParamsForNullable_Empty;
}
export interface MinterParamsForNullable_Empty {
  allowed_wrappr721_code_ids: number[];
  code_id: number;
  creation_fee: Coin;
  extension?: Empty | null;
  frozen: boolean;
  max_trading_offset_secs: number;
  min_mint_price: Coin;
  mint_fee_bps: number;
}
export interface Empty {
  [k: string]: unknown;
}