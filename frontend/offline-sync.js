// frontend/offline-sync.js
const OfflineSyncEngine = {
    dbName: "BeWellOfflineDB",
    dbVersion: 1,

    init: function() {
        return new Promise((resolve, reject) => {
            let request = indexedDB.open(this.dbName, this.dbVersion);
            request.onupgradeneeded = (e) => {
                let db = e.target.result;
                if (!db.objectStoreNames.contains("pendingClaims")) {
                    db.createObjectStore("pendingClaims", { keyPath: "id", autoIncrement: true });
                }
            };
            request.onsuccess = (e) => {
                this.db = e.target.result;
                this.registerNetworkEvents();
                resolve();
            };
            request.onerror = (e) => reject(e.target.error);
        });
    },

    // حفظ المطالبة محلياً عند انقطاع الإنترنت
    saveClaimOffline: async function(claimData) {
        let transaction = this.db.transaction(["pendingClaims"], "readwrite");
        let store = transaction.objectStore(["pendingClaims"]);
        claimData.savedAt = Date.now();
        store.add(claimData);
        console.log("⚠️ تم حفظ المطالبة الطبية محلياً بنجاح نظراً لعدم توفر إنترنت.");
    },

    // مراقبة حالة الشبكة لإعادة المزامنة التلقائية
    registerNetworkEvents: function() {
        window.addEventListener('online', () => {
            console.log("🌐 تم استعادة الاتصال بالإنترنت! بدء مزامنة البيانات المعلقة...");
            this.syncDataToServer();
        });
    },

    // رفع البيانات المخزنة محلياً لدفتر الأستاذ اللامركزي
    syncDataToServer: function() {
        let transaction = this.db.transaction(["pendingClaims"], "readwrite");
        let store = transaction.objectStore(["pendingClaims"]);
        let request = store.getAll();

        request.onsuccess = async (e) => {
            let claims = e.target.result;
            if (claims.length === 0) return;

            for (let claim of claims) {
                try {
                    let response = await fetch('/api/v1/insurance/claim-sync', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify(claim)
                    });
                    if (response.ok) {
                        // حذفها من المخزن المحلي بعد نجاح رفعها للشبكة الرئيسية
                        this.db.transaction(["pendingClaims"], "readwrite").objectStore("pendingClaims").delete(claim.id);
                        console.log(`✅ تم مزامنة ومعالجة المطالبة #${claim.id} على البلوكشين بنجاح.`);
                    }
                } catch (err) {
                    console.error("فشلت المزامنة المؤقتة، سيتم إعادة المحاولة لاحقاً:", err);
                }
            }
        };
    }
};

// تشغيل المحرك تلقائياً عند تحميل الواجهة
OfflineSyncEngine.init().catch(err => console.error("Offline DB initialization failed", err));
