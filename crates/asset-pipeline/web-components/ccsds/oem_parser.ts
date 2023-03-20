import { OemType, OemSegment } from '../../grpc/ccsds/schema/oem'

export function parseOEM(xmlDoc: Document) : OemType {

    const oem: OemType = {
        id: attrToString(xmlDoc, '/oem/@id'),
        version: attrToString(xmlDoc, '/oem/@version'),
        header: {
            comment: [attrToString(xmlDoc, '/oem/header/COMMENT')],
            creationDate: attrToString(xmlDoc, '/oem/header/CREATION_DATE'),
            originator: attrToString(xmlDoc, '/oem/header/ORIGINATOR'),
        },
        body: {
            segment: [
                parseSegment(xmlDoc),
            ],
        }
    }
    return oem
}

function parseSegment(xmlDoc: Document) : OemSegment {

    let segment: OemSegment = 
    {
        data: {
            comment: [attrToString(xmlDoc, `//segment/data/COMMENT`)],
            stateVector: [],
            covarianceMatrix: []
        },
        metadata: {
            comment: [attrToString(xmlDoc, `//segment/metadata/COMMENT`)],
            objectName: attrToString(xmlDoc, `//segment/metadata/OBJECT_NAME`),
            startTime: attrToString(xmlDoc, `//segment/metadata/START_TIME`),
            stopTime: attrToString(xmlDoc, `//segment/metadata/STOP_TIME`),
            interpolation: attrToString(xmlDoc, `//segment/metadata/INTERPOLATION`),
            interpolationDegree: attrToNumber(xmlDoc, `//segment/metadata/INTERPOLATION_DEGREE`),
            useableStartTime: attrToString(xmlDoc, `//segment/metadata/USEABLE_START_TIME`),
            useableStopTime: attrToString(xmlDoc, `//segment/metadata/USEABLE_STOP_TIME`),
            objectId: attrToString(xmlDoc, `//segment/metadata/OBJECT_ID`),
            centerName: attrToString(xmlDoc, `//segment/metadata/CENTER_NAME`),
            refFrame: attrToString(xmlDoc, `//segment/metadata/REF_FRAME`), 
            refFrameEpoch: attrToString(xmlDoc, `//segment/metadata/REF_FRAME_EPOCH`),
            timeSystem: attrToString(xmlDoc, `//segment/metadata/TIME_SYSTEM`),
        }
    }
    return segment
}

function attrToString(xmlDoc: Document, path: string): string {
    return xmlDoc.evaluate(path, xmlDoc, null, XPathResult.STRING_TYPE, null).stringValue
}

function attrToFloat(xmlDoc: Document, path: string): number {
    const s = xmlDoc.evaluate(path, xmlDoc, null, XPathResult.STRING_TYPE, null).stringValue
    return parseFloat(s)
}

function attrToNumber(xmlDoc: Document, path: string): number {
    return xmlDoc.evaluate(path, xmlDoc, null, XPathResult.NUMBER_TYPE, null).numberValue
}