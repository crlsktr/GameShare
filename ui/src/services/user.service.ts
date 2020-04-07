import { Injectable } from '@angular/core';
import { HttpService } from './http.service';
import LoginUser from './models/LoginUser';

@Injectable({
	providedIn: 'root',
})
export class UserService {
	constructor(private httpService: HttpService) {}

	public checkUsedUsername(userName: String): Promise<Boolean> {
		return this.httpService.get(`/user/checkUserName?username=${userName}`);
	}

	public createUser(newUser: LoginUser): Promise<Boolean> {
		return this.httpService.post('/user/createUser', newUser, {
			withCredentials: true,
		});
	}
}
