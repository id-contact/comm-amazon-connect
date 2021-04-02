// Set CALL_URL to something like http://proxy.irma.bellen.tweede.golf/link_phone

const http = require('https');

exports.handler = (data, _context, callback) => {
    const req = http.request(process.env.CALL_URL, {
        method: 'POST',
        headers: { 'Content-TYpe': 'application/json' },
    }, (res) => {
        res.on('end', () => {
            if (res.statusCode !== 200) {
                callback(Error(`Backed ${res.statusCode}`));
            } else {
                callback(null, {});
            }
        });
    });
    req.on('error', callback);
    req.write(JSON.stringify({
        dtmf: data.Details.ContactData.Attributes.dtmf,
        sessionid: data.Details.ContactData.ContactId
    }));
    req.end();
}
