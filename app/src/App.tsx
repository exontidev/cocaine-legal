import CustomWalletButton from "./components/wallet-ui/CustomWalletButton";

function App() {
  return (
    <div>
      <header className="bg-white border-2 p-2 px-6 fixed w-[calc(100vw-38px)] left-[38px] flex justify-between items-center">
        <h2 className="font-anton pl-2 text-3xl -skew-x-[24deg] justify-self-center">
          COCAINE.LEGAL
        </h2>
        <div className="justify-self-end">
          <CustomWalletButton />
        </div>
      </header>

      <InfiniteStripe></InfiniteStripe>
    </div>
  );
}

export default App;

const STRIPE_TEXT = "resist cocaine";
const REPEAT_COUNT = 15;

function InfiniteStripe() {
  const items = Array.from({ length: REPEAT_COUNT });

  return (
    <aside className="border-2 h-screen w-10 bg-white overflow-hidden">
      <div className="flex flex-col items-center gap-2 animate-marquee">
        {items.map((_, i) => (
          <span
            key={`a-${i}`}
            className="[writing-mode:vertical-rl] text-sm font-oxygen-mono italic shrink-0"
          >
            {STRIPE_TEXT}
          </span>
        ))}
        {items.map((_, i) => (
          <span
            key={`b-${i}`}
            className="[writing-mode:vertical-rl] text-sm font-oxygen-mono italic shrink-0"
          >
            {STRIPE_TEXT}
          </span>
        ))}
      </div>

      <style>{`
        @keyframes marquee-scroll {
          from { transform: translateY(-50%); }
          to   { transform: translateY(0); }
        }
        .animate-marquee {
          animation: marquee-scroll 20s linear infinite;
        }
      `}</style>
    </aside>
  );
}
