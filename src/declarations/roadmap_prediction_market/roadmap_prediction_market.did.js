export const idlFactory = ({ IDL }) => {
  const MarketName = IDL.Text;
  const Share = IDL.Variant({ 'No' : IDL.Null, 'Yes' : IDL.Null });
  const Response = IDL.Variant({ 'Error' : IDL.Text, 'Success' : IDL.Null });
  const Shares = IDL.Record({
    'share' : Share,
    'market' : MarketName,
    'amount' : IDL.Float64,
  });
  const Account = IDL.Record({
    'shares' : IDL.Vec(Shares),
    'tokens' : IDL.Float64,
  });
  const Market = IDL.Record({
    'name' : MarketName,
    'yes_shares' : IDL.Float64,
    'description' : IDL.Text,
    'no_shares' : IDL.Float64,
  });
  const Profile = IDL.Record({ 'name' : IDL.Text, 'description' : IDL.Text });
  return IDL.Service({
    'buy' : IDL.Func([MarketName, Share, IDL.Float64], [Response], []),
    'getAccount' : IDL.Func([], [IDL.Opt(Account)], ['query']),
    'getMarket' : IDL.Func(
        [MarketName],
        [
          IDL.Opt(
            IDL.Record({
              _0_ : Market,
              'no_price' : IDL.Float64,
              'yes_price' : IDL.Float64,
            })
          ),
        ],
        ['query'],
      ),
    'getProfile' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'getSelf' : IDL.Func([], [IDL.Opt(IDL.Tuple(Profile, Account))], ['query']),
    'join' : IDL.Func([Profile], [Response], []),
    'newMarket' : IDL.Func([MarketName, IDL.Text], [Response], []),
    'searchProfile' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'sell' : IDL.Func([MarketName, Share, IDL.Float64], [Response], []),
    'updateProfile' : IDL.Func([Profile], [Response], []),
  });
};
export const init = ({ IDL }) => { return []; };
