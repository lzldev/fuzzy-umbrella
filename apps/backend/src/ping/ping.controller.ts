import {
  Controller,
  Get,
  Injectable,
  Param,
  Post,
  UsePipes,
  ValidationPipe,
} from '@nestjs/common';
import { IsNumber, IsString } from 'class-validator';

export class Ponger {
  @IsNumber()
  id: number;
  @IsString()
  ponger: string;
}

@Injectable()
export class PingService {
  private map: Map<number, string> = new Map();

  push(id: number): string {
    this.map.set(id, new Date().toISOString());
    return this.map.get(id);
  }
  get(id: number): string | undefined {
    return this.map.get(id);
  }
  entries() {
    return Array.from(this.map.entries());
  }
}

@Controller('ping')
@UsePipes(
  new ValidationPipe({
    whitelist: true,
  }),
)
export class PingController {
  constructor(private readonly pingService: PingService) {}
  @Get()
  ping() {
    return {
      data: this.pingService.entries(),
      time: new Date().toISOString(),
    };
  }
  @Get(':id')
  pingId(@Param('id') id: number) {
    return { data: this.pingService.get(id) };
  }
  @Post(':id')
  pongCreate(@Param('id') id: number) {
    this.pingService.push(id);
    return { data: this.pingService.push(id) };
  }
}
