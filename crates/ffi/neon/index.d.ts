export type IPv4 = `${number}.${number}.${number}.${number}`;
export type Port = `${number}`;
export type SocketAddr = `${IPv4}:${Port}`;
export type TimeoutMs = number;

export interface TcpTimeout {
  connect?: TimeoutMs | null;
  read?: TimeoutMs | null;
  write?: TimeoutMs | null;
}

export interface UdpTimeout {
  read?: TimeoutMs | null;
  write?: TimeoutMs | null;
}

export interface HttpTimeout {
  global?: TimeoutMs | null;
}

export interface Timeout {
  tcp?: TcpTimeout | null;
  udp?: UdpTimeout | null;
  http?: HttpTimeout | null;
}

export type GenericDataValue = number | boolean | string | string[];
export type GenericDataMap = Record<string, GenericDataValue>;

export interface GenericPlayer {
  name: string;
  additionalData: GenericDataMap | null;
}

export interface GenericServer {
  name: string;

  description: string | null;

  map: string | null;
  mode: string | null;
  version: string | null;
  antiCheat: string | null;
  hasPassword: boolean | null;

  maxPlayers: number;
  currentPlayers: number;
  players: GenericPlayer[] | null;

  additionalData: GenericDataMap | null;
}

export default function query(
  gameId: string,
  addr: SocketAddr,
  timeout?: Timeout,
): Promise<GenericServer>;
