/* tslint:disable */
/* eslint-disable */
/**
 * endpoints
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { MapTileResponse } from './MapTileResponse';
import {
    MapTileResponseFromJSON,
    MapTileResponseFromJSONTyped,
    MapTileResponseToJSON,
    MapTileResponseToJSONTyped,
} from './MapTileResponse';

/**
 * 
 * @export
 * @interface MapResponse
 */
export interface MapResponse {
    /**
     * 
     * @type {Array<MapTileResponse>}
     * @memberof MapResponse
     */
    tiles: Array<MapTileResponse>;
}

/**
 * Check if a given object implements the MapResponse interface.
 */
export function instanceOfMapResponse(value: object): value is MapResponse {
    if (!('tiles' in value) || value['tiles'] === undefined) return false;
    return true;
}

export function MapResponseFromJSON(json: any): MapResponse {
    return MapResponseFromJSONTyped(json, false);
}

export function MapResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): MapResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'tiles': ((json['tiles'] as Array<any>).map(MapTileResponseFromJSON)),
    };
}

export function MapResponseToJSON(json: any): MapResponse {
    return MapResponseToJSONTyped(json, false);
}

export function MapResponseToJSONTyped(value?: MapResponse | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'tiles': ((value['tiles'] as Array<any>).map(MapTileResponseToJSON)),
    };
}

