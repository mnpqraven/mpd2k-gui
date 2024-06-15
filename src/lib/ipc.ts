export async function rpc() {
  const { createTauRPCProxy } = await import("../bindings/taurpc");
  return await createTauRPCProxy();
}
