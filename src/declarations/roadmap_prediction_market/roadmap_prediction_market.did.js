export const idlFactory = ({ IDL }) => {
  const Profile = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  return IDL.Service({
    'getProfile' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'getSelf' : IDL.Func([], [IDL.Opt(Profile)], ['query']),
    'searchProfile' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'updateSelf' : IDL.Func([Profile], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
