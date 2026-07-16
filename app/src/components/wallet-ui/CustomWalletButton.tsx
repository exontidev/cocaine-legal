import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "./useWalletModal";
import Dimmer from "../Dimmer";
import { useState } from "react";

export default function CustomWalletButton() {
  const { publicKey, wallet, connect, connecting, connected, disconnect } =
    useWallet();

  const { setVisible } = useWalletModal();
  const [dropdownShows, setShowDropdown] = useState(false);

  return (
    <>
      <div
        className="w-44"
        onMouseEnter={() => setShowDropdown(true)}
        onMouseLeave={() => setShowDropdown(false)}
      >
        <button
          className={`
            relative
            active:shadow-[0_0px_0px] active:translate-y-0.5 active:scale-[98%] transition-all duration-100
            flex justify-around items-center p-2
            text-sm
            bg-white
            border-2
            font-oxygen-mono
            z-50
            w-full
            cursor-pointer
            ${dropdownShows ? "border-gray-500" : "border-black"}
          `}
          onClick={() => setVisible(true)}
        >
          {wallet && (
            <img
              src={wallet.adapter.icon}
              alt={wallet.adapter.name}
              width={16}
              height={16}
            />
          )}

          {!wallet
            ? "Connect wallet"
            : connecting
              ? "Connecting..."
              : publicKey
                ? `${publicKey.toString().slice(0, 4)}...${publicKey.toString().slice(-4)}`
                : "Install wallet"}

        </button>
        {/*dropdownShows && wallet != null*/}
        <CustomWalletDropdown enabled={dropdownShows && wallet != null} />
      </div>

      <Dimmer dimmed={dropdownShows} className="z-30" />
    </>
  );
}

interface CustomWalletDropdownProps {
  enabled: boolean;
  className?: string;
}

function CustomWalletDropdown({
  className,
  enabled,
}: CustomWalletDropdownProps) {
  const { publicKey, wallet, connect, connecting, connected, disconnect } =
    useWallet();

  const { setVisible } = useWalletModal();

  return (
    <div
      className={`
      w-44 z-40 absolute top-[48px]
      bg-white
      flex justify-center items-center flex-col
      p-2
      border-2 border-gray-500
      transition-all duration-100
       ${enabled ? "opacity-100 h-[119px]" : "opacity-0 h-0 invisible pointer-events-none"}`}
    >
      <ul
        className="
        font-oxygen-mono
        flex flex-col
        "
      >
        <button className="text-sm cursor-pointer hover:skew-2 py-2 transition-all duration-100">
          Copy address
        </button>
        <button
          onClick={() => setVisible(true)}
          className="text-sm cursor-pointer hover:skew-2 py-2 transition-all duration-100"
        >
          Change Wallet
        </button>

        <button
          onClick={() => disconnect()}
          className="text-sm cursor-pointer hover:skew-2 py-2 transition-all duration-100"
        >
          Disconnect
        </button>
      </ul>
    </div>
  );
}
