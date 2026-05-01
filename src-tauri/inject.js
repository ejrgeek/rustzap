(function() {
    function applyDarkMode() {
        if (document && document.documentElement) {
            document.documentElement.classList.add('dark');
            document.documentElement.style.backgroundColor = '#111b21';
            if (document.body) {
                document.body.style.backgroundColor = '#111b21';
            }
        }
    }

    applyDarkMode();

    window.addEventListener('load', applyDarkMode);

    const observer = new MutationObserver(() => {
        if (!document.documentElement.classList.contains('dark')) {
            document.documentElement.classList.add('dark');
        }
    });
    if (document.documentElement) {
        observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
    }

    const style = document.createElement('style');
    style.textContent = `
        /* Remove banners */
        ._ak9z, ._ak9y, ._ai-p { display: none !important; }
        /* Scrollbar fina */
        ::-webkit-scrollbar { width: 6px !important; }
        ::-webkit-scrollbar-thumb { background: #374151 !important; border-radius: 10px !important; }
        ::-webkit-scrollbar-track { background: transparent !important; }
        /* Fundo escuro fixo */
        body { background-color: #111b21 !important; }
    `;
    document.head.appendChild(style);

    Object.defineProperty(window, 'Notification', {
        configurable: true,
        enumerable: true,
        writable: true,
        value: function(title, options) {
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                window.__TAURI__.invoke('send_notification', {
                    title: title,
                    body: options?.body || ''
                }).catch(function() {});
            }
        }
    });
    window.Notification.requestPermission = function() {
        return Promise.resolve('granted');
    };
    Object.defineProperty(window.Notification, 'permission', {
        get: function() { return 'granted'; }
    });
})();