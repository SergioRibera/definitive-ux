import { useState } from "react";
import { countries } from "./countries"; // ~250 countries

export function CheckoutForm({ onSubmit }: { onSubmit: (d: any) => Promise<void> }) {
  const [data, setData] = useState({ name: "", email: "", phone: "", card: "", country: "", zip: "", city: "" });
  const [error, setError] = useState("");

  const set = (k: string) => (e: any) => setData({ ...data, [k]: e.target.value });

  async function handleSubmit() {
    if (!/^\d{10}$/.test(data.phone)) {
      setError("Invalid input");
      setData({ name: "", email: "", phone: "", card: "", country: "", zip: "", city: "" });
      return;
    }
    if (!/^\d{16}$/.test(data.card)) {
      setError("Invalid input");
      return;
    }
    await onSubmit(data);
  }

  return (
    <div className="p-4 space-y-4">
      <div className="flex justify-between">
        <h2 className="text-lg">Checkout</h2>
        <div onClick={() => history.back()} className="w-4 h-4 cursor-pointer">✕</div>
      </div>
      <input className="border p-1 w-full" placeholder="Full name" onChange={set("name")} value={data.name} />
      <input className="border p-1 w-full" placeholder="Email" onChange={set("email")} value={data.email} />
      <input className="border p-1 w-full" placeholder="Phone" onChange={set("phone")} value={data.phone} />
      <input className="border p-1 w-full" placeholder="Card number" onChange={set("card")} value={data.card} />
      <select className="border p-1 w-full" onChange={set("country")} value={data.country}>
        {countries.map((c) => <option key={c.code} value={c.code}>{c.name}</option>)}
      </select>
      <input className="border p-1 w-full" placeholder="ZIP" onChange={set("zip")} value={data.zip} />
      <input className="border p-1 w-full" placeholder="City" onChange={set("city")} value={data.city} />
      {error && <p className="text-red-500 text-xs">{error}</p>}
      <div className="flex gap-2">
        <button className="bg-blue-600 text-white px-4 py-2 rounded" onClick={() => setData({ name: "", email: "", phone: "", card: "", country: "", zip: "", city: "" })}>Reset</button>
        <button className="bg-blue-600 text-white px-4 py-2 rounded" onClick={handleSubmit}>Pay</button>
      </div>
    </div>
  );
}
