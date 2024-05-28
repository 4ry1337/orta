import { Hocuspocus } from "@hocuspocus/server";
import axios from "axios";
import { Doc } from "yjs";
import dotenv from "dotenv";
dotenv.config();
const server = new Hocuspocus({
    name: "orta-colab",
    port: 4545,
    async onAuthenticate({ documentName, token }) { },
    async onStoreDocument(data) {
        axios.patch(`http://localhost:5000/api/articles/${data.documentName}/edit`, {
            content: data.document,
        });
    },
    async onLoadDocument(data) {
        const res = await axios.get(`http://localhost:5000/api/articles/${data.documentName}`);
        return createInitialDocTemplate();
    },
});
function createInitialDocTemplate() {
    return new Doc();
}
server.listen(async (payload) => {
    console.log("Server is listening on: ", payload.port);
});
