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

export interface GenericPlayer {
  id: number;
  name: string | null;
}

export interface GenericServer {
  name: string;

  description: string | null;

  map: string | null;
  mode: string | null;
  version: string | null;

  antiCheat: boolean | null;
  hasPassword: boolean | null;

  maxPlayers: number;
  currentPlayers: number;

  players: GenericPlayer[] | null;
}

export default function query(
  gameId: string,
  addr: string,
  timeout?: Timeout,
): Promise<GenericServer>;
