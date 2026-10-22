import { useState } from "react";

export function NotificationSettings({ onSave, onCancelSubscription }: any) {
  const [email, setEmail] = useState(true);
  const [sms, setSms] = useState(false);
  const [partners, setPartners] = useState(true);
  const [frequency, setFrequency] = useState("daily");
  const [showCancel, setShowCancel] = useState(false);

  return (
    <div className="max-w-xl mx-auto p-6 space-y-6">
      <h1 className="text-2xl font-bold text-indigo-700">Settings</h1>

      <p className="text-sm text-gray-500 font-semibold">Notifications</p>
      <label className="flex items-center gap-2">
        <input type="checkbox" checked={email} onChange={(e) => setEmail(e.target.checked)} /> Email updates
      </label>
      <label className="flex items-center gap-2">
        <input type="checkbox" checked={sms} onChange={(e) => setSms(e.target.checked)} /> SMS alerts
      </label>
      <p className="text-sm text-gray-500 font-semibold">Frequency</p>
      <div className="flex gap-6">
        {["realtime", "hourly", "daily", "weekly"].map((f) => (
          <span key={f} onClick={() => setFrequency(f)}
            className={frequency === f ? "text-indigo-700" : "text-gray-700"}>{f}</span>
        ))}
      </div>
      <label className="flex items-center gap-2">
        <input type="checkbox" checked={partners} onChange={(e) => setPartners(e.target.checked)} />
        Share my data with trusted partners for personalized offers
      </label>

      <div className="flex gap-3">
        <button className="bg-indigo-600 text-white px-4 py-2 rounded-lg" onClick={() => onSave({ email, sms, partners, frequency })}>Save</button>
        <button className="bg-indigo-600 text-white px-4 py-2 rounded-lg" onClick={() => setShowCancel(true)}>Cancel subscription</button>
        <button className="bg-indigo-600 text-white px-4 py-2 rounded-lg">Export data</button>
      </div>

      {showCancel && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center">
          <div className="bg-white p-6 rounded-lg space-y-4">
            <p>Are you sure? You'll lose all your saved progress and exclusive member discounts.</p>
            <button className="bg-green-600 text-white px-6 py-3 rounded-lg text-lg w-full" onClick={() => setShowCancel(false)}>Keep my benefits</button>
            <a className="text-xs text-gray-400 underline" onClick={onCancelSubscription}>No thanks, I don't care about saving money</a>
          </div>
        </div>
      )}
    </div>
  );
}
