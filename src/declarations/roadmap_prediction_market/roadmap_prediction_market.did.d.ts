import type { Principal } from '@dfinity/principal';
export type Profile = Profile_2;
export interface Profile_2 {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export interface _SERVICE {
  'get' : (arg_0: string) => Promise<Profile_2>,
  'getSelf' : () => Promise<Profile_2>,
  'search' : (arg_0: string) => Promise<[] | [Profile_2]>,
  'update' : (arg_0: Profile_2) => Promise<undefined>,
}
