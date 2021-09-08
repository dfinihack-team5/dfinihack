import type { Principal } from '@dfinity/principal';
export interface Profile {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export interface _SERVICE {
  'getProfile' : (arg_0: string) => Promise<[] | [Profile]>,
  'getSelf' : () => Promise<[] | [Profile]>,
  'searchProfile' : (arg_0: string) => Promise<[] | [Profile]>,
  'updateSelf' : (arg_0: Profile) => Promise<undefined>,
}
