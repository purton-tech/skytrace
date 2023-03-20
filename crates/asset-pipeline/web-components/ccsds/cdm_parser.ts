import { CdmType, ObjectType, ReferenceFrameType, DvUnits, ScreenVolumeShapeType, ScreenVolumeFrameType, CdmSegment, ManeuverableType, CovarianceMethodType } from '../../grpc/ccsds/schema/cdm'
import { YesNoType, ObjectDescriptionType, LengthType, DayIntervalType, PercentageType, LengthUnits } from '../../grpc/ccsds/schema/common'

export function parseCDM(xmlDoc: Document) : CdmType {

    const cdm: CdmType = {
        id: attrToString(xmlDoc, '/cdm/@id'),
        version: attrToString(xmlDoc, '/cdm/@version'),
        header: {
            comment: [attrToString(xmlDoc, '/cdm/header/COMMENT')],
            creationDate: attrToString(xmlDoc, '/cdm/header/CREATION_DATE'),
            originator: attrToString(xmlDoc, '/cdm/header/ORIGINATOR'),
            messageFor: attrToString(xmlDoc, '/cdm/header/MESSAGE_FOR'),
            messageId: attrToString(xmlDoc, '/cdm/header/MESSAGE_ID')
        },
        body: {
            relativeMetadataData: {
                relativeStateVector: {
                    relativePositionR: {
                        value: attrToNumber(xmlDoc, '//RELATIVE_POSITION_R'),
                        units: LengthUnits.LENGTH_UNITS_M,
                    },
                    relativePositionT: {
                        value: attrToNumber(xmlDoc, '//RELATIVE_POSITION_T'),
                        units: LengthUnits.LENGTH_UNITS_M,
                    },
                    relativePositionN: {
                        value: attrToNumber(xmlDoc, '//RELATIVE_POSITION_N'),
                        units: LengthUnits.LENGTH_UNITS_M,
                    },
                    relativeVelocityR: {
                        value: attrToNumber(xmlDoc, '//RELATIVE_VELOCITY_R'),
                        units: DvUnits.M_S,
                    },
                    relativeVelocityT: {
                        value: attrToNumber(xmlDoc, '//RELATIVE_VELOCITY_T'),
                        units: DvUnits.M_S,
                    },
                    relativeVelocityN: {
                        value: attrToNumber(xmlDoc, '//RELATIVE_VELOCITY_N'),
                        units: DvUnits.M_S,
                    },
                },
                comment: [attrToString(xmlDoc, `//relativeMetadataData/COMMENT`)],
                missDistance: {
                    value: attrToNumber(xmlDoc, '/c/relativeMetadataData/MISS_DISTANCE'),
                    units: LengthUnits.LENGTH_UNITS_M,
                }, 
                tca: attrToString(xmlDoc, `//relativeMetadataData/TCA`), 
                startScreenPeriod: attrToString(xmlDoc, `//relativeMetadataData/START_SCREEN_PERIOD`),
                stopScreenPeriod: attrToString(xmlDoc, `//relativeMetadataData/STOP_SCREEN_PERIOD`),
                screenVolumeFrame: ScreenVolumeFrameType[attrToString(xmlDoc, `//relativeMetadataData/SCREEN_VOLUME_FRAME`)],
                screenVolumeShape: ScreenVolumeShapeType[attrToString(xmlDoc, `//relativeMetadataData/SCREEN_VOLUME_SHAPE`)], 
                screenEntryTime: attrToString(xmlDoc, `//relativeMetadataData/SCREEN_ENTRY_TIME`), 
                screenExitTime: attrToString(xmlDoc, `//relativeMetadataData/SCREEN_EXIT_TIME`),
                collisionProbability: attrToFloat(xmlDoc, `//relativeMetadataData/COLLISION_PROBABILITY`),
                collisionProbabilityMethod: attrToString(xmlDoc, `//relativeMetadataData/COLLISION_PROBABILITY_METHOD`)
            },
            segment: [
                parseSegment(xmlDoc, 1),
                parseSegment(xmlDoc, 2),
            ],
        }
    }
    return cdm
}

function parseSegment(xmlDoc: Document, index: number) : CdmSegment {

    let segment: CdmSegment = 
    {
        data: {
            comment: [attrToString(xmlDoc, `//segment[${index}]/data/COMMENT`)],
            odParameters: {
                comment: [attrToString(xmlDoc, `//segment[${index}]/data/odParameters/COMMENT`)],
                timeLastobStart: attrToString(xmlDoc, `//segment[${index}]/data/odParameters/TIME_LASTOB_START`), 
                timeLastobEnd: attrToString(xmlDoc, `//segment[${index}]/data/odParameters/TIME_LASTOB_END`), 
                obsAvailable: attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/OBS_AVAILABLE`), 
                obsUsed: attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/OBS_USED`),  
                tracksAvailable: attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/TRACKS_AVAILABLE`), 
                tracksUsed: attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/TRACKS_USED`),  
                weightedRms: attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/WEIGHTED_RMS`),
                // Optional
                residualsAccepted: PercentageType[attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/RESIDUALS_ACCEPTED`)],
                actualOdSpan: DayIntervalType[attrToNumber(xmlDoc, `//segment[${index}]/data/odParameters/WEIGHTED_RMS`)],
            },
            additionalParameters: {
                comment: [attrToString(xmlDoc, `//segment[${index}]/data/additionalParameters/COMMENT`)],
            },
            stateVector: {
                comment: [attrToString(xmlDoc, `//segment[${index}]/data/stateVector/COMMENT`)],
            },
            covarianceMatrix: {
                comment: [attrToString(xmlDoc, `//segment[${index}]/data/covarianceMatrix/COMMENT`)],
            }
        },
        metadata: {
            comment: [attrToString(xmlDoc, `//segment[${index}]/metadata/COMMENT`)],
            object: ObjectType[attrToString(xmlDoc, `//segment[${index}]/metadata/OBJECT`)] || 0,
            objectName: attrToString(xmlDoc, `//segment[${index}]/metadata/OBJECT_NAME`),
            catalogName: attrToString(xmlDoc, `//segment[${index}]/metadata/CATALOG_NAME`),
            objectDesignator: attrToString(xmlDoc, `//segment[${index}]/metadata/OBJECT_DESIGNATOR`),
            internationalDesignator: attrToString(xmlDoc, `//segment[${index}]/metadata/INTERNATIONAL_DESIGNATOR`),
            objectType: ObjectDescriptionType[attrToString(xmlDoc, `//segment[${index}]/metadata/OBJECT_TYPE`)],
            operatorContactPosition: attrToString(xmlDoc, `//segment[${index}]/metadata/OPERATOR_CONTACT_POSITION`),
            operatorOrganization: attrToString(xmlDoc, `//segment[${index}]/metadata/OPERATOR_ORGANISATION`),
            operatorPhone: attrToString(xmlDoc, `//segment[${index}]/metadata/OPERATOR_PHONE`),
            operatorEmail: attrToString(xmlDoc, `//segment[${index}]/metadata/OPERATOR_EMAIL`),
            ephemerisName: attrToString(xmlDoc, `//segment[${index}]/metadata/EPHERIS_NAME`),
            covarianceMethod: CovarianceMethodType[attrToString(xmlDoc, `//segment[${index}]/metadata/COVARIANCE_METHOD`)],
            maneuverable: ManeuverableType[attrToString(xmlDoc, `//segment[${index}]/metadata/MANEUVERABLE`)],
            orbitCenter: attrToString(xmlDoc, `//segment[${index}]/metadata/ORBIT_CENTER`), 
            refFrame: ReferenceFrameType[attrToString(xmlDoc, `//segment[${index}]/metadata/REF_FRAME`)] || 0, 
            gravityModel: attrToString(xmlDoc, `//segment[${index}]/metadata/GRAVITY_MODEL`), 
            atmosphericModel: attrToString(xmlDoc, `//segment[${index}]/metadata/ATMOSPHERIC_MODEL`),
            nBodyPerturbations: attrToString(xmlDoc, `//segment[${index}]/metadata/N_BODY_PERTURBATIONS`), 
            solarRadPressure: YesNoType[attrToString(xmlDoc, `//segment[${index}]/metadata/SOLAR_RAD_PRESSURE`)], 
            earthTides: YesNoType[attrToString(xmlDoc, `//segment[${index}]/metadata/EARTH_TIDES`)], 
            intrackThrust: YesNoType[attrToString(xmlDoc, `//segment[${index}]/metadata/INTRACK_THRUST`)]
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