import { OpmType } from '../../grpc/ccsds/schema/opm'
import { AngleUnits, PositionUnits } from '../../grpc/ccsds/schema/common'

export function parseOPM(xmlDoc: Document) : OpmType {

    const opm: OpmType = {
        id: attrToString(xmlDoc, '/opm/@id'),
        version: attrToString(xmlDoc, '/opm/@version'),
        header: {
            comment: [attrToString(xmlDoc, '/opm/header/COMMENT')],
            creationDate: attrToString(xmlDoc, '/opm/header/CREATION_DATE'),
            originator: attrToString(xmlDoc, '/opm/header/ORIGINATOR'),
        },
        body: {
            segment: {
                metadata: {
                    comment: [attrToString(xmlDoc, `//segment/metadata/COMMENT`)],
                    objectName: attrToString(xmlDoc, `//segment/metadata/OBJECT_NAME`),
                    objectId: attrToString(xmlDoc, `//segment/metadata/OBJECT_ID`),
                    centerName: attrToString(xmlDoc, `//segment/metadata/CENTER_NAME`),
                    refFrame: attrToString(xmlDoc, `//segment/metadata/REF_FRAME`), 
                    refFrameEpoch: attrToString(xmlDoc, `//segment/metadata/REF_FRAME_EPOCH`),
                    timeSystem: attrToString(xmlDoc, `//segment/metadata/TIME_SYSTEM`),
                },
                data: {
                    comment: [attrToString(xmlDoc, '//segment/data/COMMENT')],
                    maneuverParameters: [],
                }
            },
        }
    }

    if(xmlDoc.querySelector('keplerianElements')) {

        opm['body']!['segment']!['data']!['keplerianElements'] = {
            comment: [attrToString(xmlDoc, '//keplerianElements/COMMENT')],
            argOfPericenter: {
                value: attrToNumber(xmlDoc, '//keplerianElements/ARG_OF_PERICENTER'),
                units: AngleUnits.DEG,
            },
            eccentricity: attrToFloat(xmlDoc, '//keplerianElements/ECCENTRICITY"'),
            inclination: {
                value: attrToNumber(xmlDoc, '//keplerianElements/INCLINATION'),
                units: AngleUnits.DEG,
            },
            meanAnomaly: {
                value: attrToNumber(xmlDoc, '//keplerianElements/MEAN_ANOMALY'),
                units: AngleUnits.DEG,
            },
            trueAnomaly: {
                value: attrToNumber(xmlDoc, '//keplerianElements/TRUE_ANOMALY'),
                units: AngleUnits.DEG,
            },
            raOfAscNode: {
                value: attrToNumber(xmlDoc, '//keplerianElements/RA_OF_ASC_NODE'),
                units: AngleUnits.DEG,
            },
            semiMajorAxis: {
                value: attrToNumber(xmlDoc, '//keplerianElements/SEMI_MAJOR_AXIS'),
                units: PositionUnits.KM,
            }
        }
    }

    return opm
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