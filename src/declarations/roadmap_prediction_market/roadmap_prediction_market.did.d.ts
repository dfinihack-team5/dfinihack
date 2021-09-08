import type { Principal } from '@dfinity/principal';
export interface Account { 'tokens' : number }
export interface Profile {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export type Response = { 'Error' : string } |
  { 'Success' : null };
export interface _SERVICE {
  'getAccount' : () => Promise<[] | [Account]>,
  'getProfile' : (arg_0: string) => Promise<[] | [Profile]>,
  'getSelf' : () => Promise<[] | [[Profile, Account]]>,
  'join' : (arg_0: Profile) => Promise<Response>,
  'searchProfile' : (arg_0: string) => Promise<[] | [Profile]>,
  'updateProfile' : (arg_0: Profile) => Promise<Response>,
}
