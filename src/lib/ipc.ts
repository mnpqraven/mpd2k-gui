import { createTauRPCProxy } from '../bindings/taurpc';

const rpc = await createTauRPCProxy();

export { rpc };
