export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({ 'tokens' : IDL.Float64 });
  const Profile = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  const Response = IDL.Variant({ 'Error' : IDL.Text, 'Success' : IDL.Null });
  return IDL.Service({
    'getAccount' : IDL.Func([], [IDL.Opt(Account)], ['query']),
    'getProfile' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'getSelf' : IDL.Func([], [IDL.Opt(IDL.Tuple(Profile, Account))], ['query']),
    'join' : IDL.Func([Profile], [Response], []),
    'searchProfile' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'updateProfile' : IDL.Func([Profile], [Response], []),
  });
};
export const init = ({ IDL }) => { return []; };
