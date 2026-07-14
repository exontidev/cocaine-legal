import { WalletReadyState } from "@solana/wallet-adapter-base";
import type { Wallet } from "@solana/wallet-adapter-react";
import type { FC, MouseEventHandler } from "react";
import React from "react";
import { Button } from "./Button.js";
import { WalletIcon } from "./WalletIcon.js";

export interface WalletListItemProps {
  handleClick: MouseEventHandler<HTMLButtonElement>;
  tabIndex?: number;
  wallet: Wallet;
}

export const WalletListItem: FC<WalletListItemProps> = ({
  handleClick,
  tabIndex,
  wallet,
}) => {
  return (
    <li className="flex justify-center items-center">
      <Button
        className="flex items-center px-10 py-2.5 w-[95%] hover:bg-gray-100 hover:-skew-x-6 active:shadow-[0_0px_0px] bg-gray-50 border-2 mt-1.5 border-black cursor-pointer transition-all active:scale-[102%] duration-100"
        onClick={handleClick}
        startIcon={<WalletIcon wallet={wallet} />}
        tabIndex={tabIndex}
      >
        {wallet.adapter.name}
        {wallet.readyState === WalletReadyState.Installed && (
          <span className="ml-5 text-gray-400 italic">(Detected)</span>
        )}
      </Button>
    </li>
  );
};
