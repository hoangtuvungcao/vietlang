document.addEventListener('DOMContentLoaded', async () => {
  try {
    const res = await fetch('/api/health');
    const data = await res.json();
    document.getElementById('server-status').innerHTML = 
      `<span style="color:#10b981; font-weight:bold;">🟢 Hệ Thống Hoạt Động</span> — Engine: <b>${data.data.engine}</b>`;
  } catch (e) {
    document.getElementById('server-status').innerHTML = '<span style="color:#ef4444;">🔴 Không thể kết nối đến máy chủ</span>';
  }
});
