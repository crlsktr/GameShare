import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { HttpService } from './http-service.service';

@Injectable({
	providedIn: 'root'
})
export class UserServiceService {

	constructor(private httpService: HttpService) { }

	public checkUsedUsername (userName: String) : Promise<Boolean> {
		return this.httpService.post("/user/checkUserName", {username: userName})
	}
}
