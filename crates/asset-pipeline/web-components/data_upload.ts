import { SideDrawer } from './side-drawer';
import { parseCDM } from './ccsds/cdm_parser'
import { parseOEM } from './ccsds/oem_parser'
import { parseOPM } from './ccsds/opm_parser'
import { CdmType } from '../grpc/ccsds/schema/cdm'
import { OpmType } from '../grpc/ccsds/schema/opm'
import { OemType } from '../grpc/ccsds/schema/oem'
import { ECDSAKeyPair, ByteData } from "../cryptography/vault"

export class DataUpload extends SideDrawer {

    readonly uploadFieldsTarget = document.getElementById('upload-fields') as HTMLFieldSetElement
    readonly verifyFieldsTarget = document.getElementById('verify-fields') as HTMLFieldSetElement
    readonly verifyTableTarget = document.getElementById('verify-table') as HTMLTableSectionElement
    readonly signatureTarget = document.getElementById('signature') as HTMLInputElement
    readonly signatureViewTarget = document.getElementById('signature-view') as HTMLParagraphElement
    readonly submitButtonTarget = document.getElementById('submit-button') as HTMLButtonElement
    readonly fileFieldTarget = document.getElementById('file-field') as HTMLInputElement
    readonly payloadFieldTarget = document.getElementById('payload') as HTMLInputElement
    readonly payloadTypeFieldTarget = document.getElementById('payloadtype') as HTMLInputElement
    readonly payloadFormTarget = document.getElementById('payload-form') as HTMLFormElement

    constructor() {
        super()
            this.fileFieldTarget.onchange = e => {
                if (this.fileFieldTarget.files && this.fileFieldTarget.files[0]) {
                    this.parseFileUpload(this.fileFieldTarget.files[0])
                }
            }
    }

    async parseFileUpload(file: Blob) {
        var reader = new FileReader()
        const thiz = this
        reader.onload = async function () {
            const parser = new DOMParser()
            if(reader && reader.result) {

                const xmlString = reader.result.toString()
                const doc = parser.parseFromString(xmlString, "application/xml")

                const dataToSign = xmlToByteData(xmlString, doc)

                if(dataToSign != null) {
        
                    const ecdsaKeyPair = await ECDSAKeyPair.fromBarricade()
                    const protoSignature = await ecdsaKeyPair.privateKey.sign(dataToSign.data)
        
                    thiz.signatureViewTarget.innerText = protoSignature.toDER().hex
        
                    thiz.showVerifyScreen(dataToSign.html)
        
                    // Add to out form as base64 encoded string
                    thiz.payloadFieldTarget.value = dataToSign.data.b64
                    thiz.payloadTypeFieldTarget.value = dataToSign.type
                    thiz.signatureTarget.value = protoSignature.toDER().hex
                }
            }

        }
        reader.readAsText(file);
    }

    showVerifyScreen(html: string) {
        this.verifyTableTarget.innerHTML = html
        this.verifyFieldsTarget.classList.remove('d-none');
        this.uploadFieldsTarget.classList.add('d-none')
        this.submitButtonTarget.classList.remove('d-none')
    }
}

function xmlToByteData(xmlString: string, doc: Document) : { data: ByteData, html: string, type: string} | null {

    if(xmlString.indexOf('<cdm ') != -1) {
        console.log('Parse CDM')
        let cdm = parseCDM(doc)
        console.log(cdm)
        if(cdm != null && cdm.header != null) {
            const html = `<tr><th>Type</th><td>Conjunction Data Message</td></tr>`
                + `<tr><th>Comment</th><td>${cdm.header.comment}</td></tr>`
                + `<tr><th>Creation Date</th><td>${cdm.header.creationDate}</td></tr>`
                + `<tr><th>Originator</th><td>${cdm.header.originator}</td></tr>`
                + `<tr><th>Message For</th><td>${cdm.header.messageFor}</td></tr>`
                + `<tr><th>Message ID</th><td>${cdm.header.messageId}</td></tr>`
            const cdmAsProto = CdmType.toBinary(cdm)
            return { data: new ByteData(cdmAsProto), html: html, type: 'CDM' }
        }
    } else if(xmlString.indexOf('<oem ') != -1) {
        console.log('Parse OEM')
        let oem = parseOEM(doc)
        console.log(oem)
        if(oem != null && oem.header != null) {
            const html = `<tr><th>Type</th><td>Conjunction Data Message</td></tr>`
                + `<tr><th>Comment</th><td>${oem.header.comment}</td></tr>`
                + `<tr><th>Creation Date</th><td>${oem.header.creationDate}</td></tr>`
                + `<tr><th>Originator</th><td>${oem.header.originator}</td></tr>`
            const cdmAsProto = OemType.toBinary(oem)
            return { data: new ByteData(cdmAsProto), html: html, type: 'OEM'  }
        }
    } else if(xmlString.indexOf('<opm ') != -1) {
        console.log('Parse OPM')
        let opm = parseOPM(doc)
        console.log(opm)
        if(opm != null && opm.header != null) {
            const html = `<tr><th>Type</th><td>Conjunction Data Message</td></tr>`
                + `<tr><th>Comment</th><td>${opm.header.comment}</td></tr>`
                + `<tr><th>Creation Date</th><td>${opm.header.creationDate}</td></tr>`
                + `<tr><th>Originator</th><td>${opm.header.originator}</td></tr>`
            const cdmAsProto = OpmType.toBinary(opm)
            return { data: new ByteData(cdmAsProto), html: html, type: 'OPM'  }
        }
    }
    console.log('Should not get here')
    return null
}

customElements.define('data-upload', DataUpload)