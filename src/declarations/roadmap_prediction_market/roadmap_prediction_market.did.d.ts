import type { Principal } from '@dfinity/principal';
export interface Account { 'shares' : Array<Shares>, 'tokens' : number }
export interface Market {
  'name' : MarketName,
  'yes_shares' : number,
  'description' : string,
  'no_shares' : number,
}
export type MarketName = string;
export interface Profile { 'name' : string, 'description' : string }
export type Response = { 'Error' : string } |
  { 'Success' : null };
export type Share = { 'No' : null } |
  { 'Yes' : null };
export interface Shares {
  'share' : Share,
  'market' : MarketName,
  'amount' : number,
}
export interface _SERVICE {
  'buy' : (arg_0: MarketName, arg_1: Share, arg_2: number) => Promise<Response>,
  'getAccount' : () => Promise<[] | [Account]>,
  'getMarket' : (arg_0: MarketName) => Promise<
      [] | [{ _0_ : Market, 'no_price' : number, 'yes_price' : number }]
    >,
  'getProfile' : (arg_0: string) => Promise<[] | [Profile]>,
  'getSelf' : () => Promise<[] | [[Profile, Account]]>,
  'join' : (arg_0: Profile) => Promise<Response>,
  'newMarket' : (arg_0: MarketName, arg_1: string) => Promise<Response>,
  'searchProfile' : (arg_0: string) => Promise<[] | [Profile]>,
  'sell' : (arg_0: MarketName, arg_1: Share, arg_2: number) => Promise<
      Response
    >,
  'updateProfile' : (arg_0: Profile) => Promise<Response>,
}
